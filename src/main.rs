use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use loinc_validator_rs::cli::{Args, OutputFormat};
use loinc_validator_rs::notes::{get_loinc_note, get_unit_note};
use loinc_validator_rs::output::{JsonRecordOutput, ProcessedOutput, write_outputs};
use loinc_validator_rs::stats::ValidationStats;
use loinc_validator_rs::validator::{LoincValidator, LoincVldStatus, UnitVldStatus};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};

const MAPPING_JSON: &str = include_str!("data/unit_to_ucum_mapping.json");
const LOINC_JSON: &str = include_str!("data/loinc_unit.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let validator = LoincValidator::new_with_strict(LOINC_JSON, MAPPING_JSON, args.strict)?;

    let file = File::open(&args.input_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();

    let target_loinc_col = args.loinc_column.to_lowercase();
    let loinc_idx = headers
        .iter()
        .position(|h| h.to_lowercase() == target_loinc_col)
        .ok_or_else(|| format!("Column '{}' not found in CSV headers", args.loinc_column))?;

    let target_unit_col = args.unit_column.to_lowercase();
    let unit_idx = headers
        .iter()
        .position(|h| h.to_lowercase() == target_unit_col)
        .ok_or_else(|| format!("Column '{}' not found in CSV headers", args.unit_column))?;

    let writer_box: Box<dyn Write> = match args.output_file {
        Some(ref path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    let mut writer = BufWriter::new(writer_box);

    if args.format == OutputFormat::Csv {
        let mut out_headers = headers.clone();
        out_headers.push_field("LMV_UNIT_STATUS");
        out_headers.push_field("LMV_LOINC_STATUS");
        out_headers.push_field("LMV_SUBSTITUTED_UNIT");
        out_headers.push_field("LMV_UNIT_NOTE");
        out_headers.push_field("LMV_LOINC_NOTE");
        let mut csv_wtr = csv::Writer::from_writer(&mut writer);
        csv_wtr.write_record(&out_headers)?;
    }

    let pb = if args.no_progress {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] Processed {pos} rows ({per_sec})")
                .unwrap(),
        );
        pb
    };

    const CHUNK_SIZE: usize = 10_000;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);
    let mut global_stats = ValidationStats::default();

    for result in rdr.records() {
        let record = result?;
        buffer.push(record);
        if buffer.len() >= CHUNK_SIZE {
            let (stats, outputs) = process_chunk(
                &buffer,
                &validator,
                &headers,
                loinc_idx,
                unit_idx,
                args.format,
            );
            global_stats.merge(&stats);
            write_outputs(&mut writer, &outputs, args.format)?;
            pb.inc(buffer.len() as u64);
            buffer.clear();
        }
    }

    if !buffer.is_empty() {
        let (stats, outputs) = process_chunk(
            &buffer,
            &validator,
            &headers,
            loinc_idx,
            unit_idx,
            args.format,
        );
        global_stats.merge(&stats);
        write_outputs(&mut writer, &outputs, args.format)?;
        pb.inc(buffer.len() as u64);
    }

    pb.finish_with_message("Processing complete!");
    writer.flush()?;

    global_stats.print_report();
    Ok(())
}

fn process_chunk(
    records: &[csv::StringRecord],
    validator: &LoincValidator,
    headers: &csv::StringRecord,
    loinc_idx: usize,
    unit_idx: usize,
    format: OutputFormat,
) -> (ValidationStats, Vec<ProcessedOutput>) {
    let header_strs: Vec<&str> = headers.iter().collect();

    records
        .par_iter()
        .fold(
            || {
                (
                    ValidationStats::default(),
                    Vec::with_capacity(records.len()),
                )
            },
            |(mut stats, mut outputs), record| {
                let loinc = record.get(loinc_idx).unwrap_or_default();
                let unit = record.get(unit_idx).unwrap_or_default();

                let val_res = validator.validate_loinc_unit(loinc, unit);

                match val_res.unit_status {
                    UnitVldStatus::VALID => stats.valid_units += 1,
                    UnitVldStatus::InvalidFixed => stats.invalid_fixed_units += 1,
                    UnitVldStatus::InvalidUnknown => stats.invalid_unknown_units += 1,
                    UnitVldStatus::MissingUnit => stats.missing_units += 1,
                }
                match val_res.loinc_status {
                    Some(LoincVldStatus::CORRECT) => stats.correct_loinc += 1,
                    Some(LoincVldStatus::INCORRECT) => stats.incorrect_loinc += 1,
                    Some(LoincVldStatus::UNKNOWN) => stats.unknown_loinc += 1,
                    Some(LoincVldStatus::MissingLoinc) => stats.missing_loinc += 1,
                    None => {}
                }

                let unit_status_str = val_res.unit_status.as_str();
                let loinc_status_str = val_res.loinc_status.map(|s| s.as_str()).unwrap_or("");
                let substituted_unit = val_res.substituted_unit.as_deref().unwrap_or("");
                let unit_note = get_unit_note(val_res.unit_status);
                let loinc_note = get_loinc_note(val_res.loinc_status);

                match format {
                    OutputFormat::Csv => {
                        let mut row: Vec<String> = record.iter().map(String::from).collect();
                        row.push(unit_status_str.to_string());
                        row.push(loinc_status_str.to_string());
                        row.push(substituted_unit.to_string());
                        row.push(unit_note.to_string());
                        row.push(loinc_note.to_string());
                        outputs.push(ProcessedOutput::CsvRow(row));
                    }
                    OutputFormat::Json => {
                        let mut map = HashMap::new();
                        for (h, f) in header_strs.iter().zip(record.iter()) {
                            map.insert(*h, f);
                        }
                        let json_rec = JsonRecordOutput {
                            original: map,
                            lmv_unit_status: unit_status_str,
                            lmv_loinc_status: loinc_status_str,
                            lmv_substituted_unit: substituted_unit,
                            lmv_unit_note: unit_note,
                            lmv_loinc_note: loinc_note,
                        };
                        if let Ok(json_str) = serde_json::to_string(&json_rec) {
                            outputs.push(ProcessedOutput::JsonString(json_str));
                        }
                    }
                }

                (stats, outputs)
            },
        )
        .reduce(
            || (ValidationStats::default(), Vec::new()),
            |(mut stats_a, mut outputs_a), (stats_b, mut outputs_b)| {
                stats_a.merge(&stats_b);
                outputs_a.append(&mut outputs_b);
                (stats_a, outputs_a)
            },
        )
}
