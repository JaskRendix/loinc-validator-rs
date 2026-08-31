use loinc_validator_rs::cli::OutputFormat;
use loinc_validator_rs::output::{ProcessedOutput, write_outputs};

#[test]
fn write_empty_csv_outputs() {
    let mut buffer = Vec::new();
    let outputs: Vec<ProcessedOutput> = vec![];

    write_outputs(&mut buffer, &outputs, OutputFormat::Csv).unwrap();

    // CSV writer should produce nothing for empty input
    assert!(buffer.is_empty());
}

#[test]
fn write_single_csv_row() {
    let mut buffer = Vec::new();
    let outputs = vec![ProcessedOutput::CsvRow(vec![
        "A".into(),
        "B".into(),
        "C".into(),
    ])];

    write_outputs(&mut buffer, &outputs, OutputFormat::Csv).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result.trim(), "A,B,C");
}

#[test]
fn write_multiple_csv_rows() {
    let mut buffer = Vec::new();
    let outputs = vec![
        ProcessedOutput::CsvRow(vec!["1".into(), "2".into()]),
        ProcessedOutput::CsvRow(vec!["3".into(), "4".into()]),
    ];

    write_outputs(&mut buffer, &outputs, OutputFormat::Csv).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "1,2");
    assert_eq!(lines[1], "3,4");
}

#[test]
fn write_empty_json_outputs() {
    let mut buffer = Vec::new();
    let outputs: Vec<ProcessedOutput> = vec![];

    write_outputs(&mut buffer, &outputs, OutputFormat::Json).unwrap();

    assert!(buffer.is_empty());
}

#[test]
fn write_single_json_string() {
    let mut buffer = Vec::new();
    let outputs = vec![ProcessedOutput::JsonString("{\"a\":1}".into())];

    write_outputs(&mut buffer, &outputs, OutputFormat::Json).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result.trim(), "{\"a\":1}");
}

#[test]
fn write_multiple_json_strings() {
    let mut buffer = Vec::new();
    let outputs = vec![
        ProcessedOutput::JsonString("{\"x\":1}".into()),
        ProcessedOutput::JsonString("{\"y\":2}".into()),
    ];

    write_outputs(&mut buffer, &outputs, OutputFormat::Json).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "{\"x\":1}");
    assert_eq!(lines[1], "{\"y\":2}");
}

#[test]
fn write_mixed_outputs_csv_mode_ignores_json() {
    let mut buffer = Vec::new();
    let outputs = vec![
        ProcessedOutput::CsvRow(vec!["A".into()]),
        ProcessedOutput::JsonString("{\"ignored\":true}".into()),
    ];

    write_outputs(&mut buffer, &outputs, OutputFormat::Csv).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result.trim(), "A");
}

#[test]
fn write_mixed_outputs_json_mode_ignores_csv() {
    let mut buffer = Vec::new();
    let outputs = vec![
        ProcessedOutput::JsonString("{\"ok\":1}".into()),
        ProcessedOutput::CsvRow(vec!["ignored".into()]),
    ];

    write_outputs(&mut buffer, &outputs, OutputFormat::Json).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result.trim(), "{\"ok\":1}");
}

#[test]
fn write_outputs_does_not_panic_on_large_input() {
    let mut buffer = Vec::new();
    let mut outputs = Vec::new();

    for i in 0..10_000 {
        outputs.push(ProcessedOutput::JsonString(format!("{{\"n\":{}}}", i)));
    }

    write_outputs(&mut buffer, &outputs, OutputFormat::Json).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();

    assert_eq!(lines.len(), 10_000);
}

#[test]
fn write_jsonl_outputs() {
    let mut buffer = Vec::new();
    let outputs = vec![
        ProcessedOutput::JsonString("{\"a\":1}".into()),
        ProcessedOutput::JsonString("{\"b\":2}".into()),
    ];

    write_outputs(&mut buffer, &outputs, OutputFormat::Jsonl).unwrap();

    let result = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "{\"a\":1}");
    assert_eq!(lines[1], "{\"b\":2}");
}
