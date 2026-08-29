use crate::cli::OutputFormat;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;

#[derive(Serialize)]
pub struct JsonRecordOutput<'a> {
    #[serde(flatten)]
    pub original: HashMap<&'a str, &'a str>,
    pub lmv_unit_status: &'static str,
    pub lmv_loinc_status: &'static str,
    pub lmv_substituted_unit: &'a str,
    pub lmv_unit_note: &'static str,
    pub lmv_loinc_note: &'static str,
}

#[derive(Debug)]
pub enum ProcessedOutput {
    CsvRow(Vec<String>),
    JsonString(String),
}

pub fn write_outputs<W: Write>(
    writer: &mut W,
    outputs: &[ProcessedOutput],
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Csv => {
            let mut csv_wtr = csv::Writer::from_writer(writer);
            for out in outputs {
                if let ProcessedOutput::CsvRow(row) = out {
                    csv_wtr.write_record(row)?;
                }
            }
            csv_wtr.flush()?;
        }
        OutputFormat::Json => {
            for out in outputs {
                if let ProcessedOutput::JsonString(json_str) = out {
                    writer.write_all(json_str.as_bytes())?;
                    writer.write_all(b"\n")?;
                }
            }
        }
    }
    Ok(())
}
