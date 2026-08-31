use clap::Parser;
use loinc_validator_rs::cli::{Args, OutputFormat};

#[test]
fn test_default_values() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
    ]);

    assert_eq!(args.input_file, "in.csv");
    assert_eq!(args.loinc_column, "loinc");
    assert_eq!(args.unit_column, "unit");

    // Defaults
    assert!(!args.strict);
    assert_eq!(args.format, OutputFormat::Csv);
    assert!(!args.no_progress);
    assert!(args.output_file.is_none());
}

#[test]
fn test_output_file_parsing() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--output-file",
        "out.csv",
    ]);

    assert_eq!(args.output_file.as_deref(), Some("out.csv"));
}

#[test]
fn test_strict_flag() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--strict",
    ]);

    assert!(args.strict);
}

#[test]
fn test_no_progress_flag() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--no-progress",
    ]);

    assert!(args.no_progress);
}

#[test]
fn test_format_json() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--format",
        "json",
    ]);

    assert_eq!(args.format, OutputFormat::Json);
}

#[test]
fn test_format_csv_explicit() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--format",
        "csv",
    ]);

    assert_eq!(args.format, OutputFormat::Csv);
}

#[test]
fn test_format_invalid_value() {
    let result = Args::try_parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--format",
        "xml", // invalid
    ]);

    assert!(result.is_err());
}

#[test]
fn test_all_flags_together() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--output-file",
        "out.json",
        "--strict",
        "--format",
        "json",
        "--no-progress",
    ]);

    assert_eq!(args.input_file, "in.csv");
    assert_eq!(args.loinc_column, "loinc");
    assert_eq!(args.unit_column, "unit");
    assert_eq!(args.output_file.as_deref(), Some("out.json"));
    assert!(args.strict);
    assert_eq!(args.format, OutputFormat::Json);
    assert!(args.no_progress);
}

#[test]
fn test_short_flags() {
    let args = Args::parse_from([
        "prog", "-i", "in.csv", "-l", "loinc", "-u", "unit", "-o", "out.csv", "-s", "-f", "json",
    ]);

    assert_eq!(args.input_file, "in.csv");
    assert_eq!(args.loinc_column, "loinc");
    assert_eq!(args.unit_column, "unit");
    assert_eq!(args.output_file.as_deref(), Some("out.csv"));
    assert!(args.strict);
    assert_eq!(args.format, OutputFormat::Json);
}

#[test]
fn test_new_cli_flags() {
    let args = Args::parse_from([
        "prog",
        "--input-file",
        "in.csv",
        "--loinc-column",
        "loinc",
        "--unit-column",
        "unit",
        "--enable-canonicalization",
        "--enable-suggestions",
        "--format",
        "jsonl",
        "--stats-output",
        "stats.json",
    ]);

    assert!(args.enable_canonicalization);
    assert!(args.enable_suggestions);
    assert_eq!(args.format, OutputFormat::Jsonl);
    assert_eq!(args.stats_output.as_deref(), Some("stats.json"));
}
