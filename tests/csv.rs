use loinc_validator_rs::validator::get_validator;
use std::fs::File;
use std::path::PathBuf;

#[test]
fn test_sample_csv_processing() {
    let validator = get_validator().unwrap();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let candidate_paths = vec![
        PathBuf::from(manifest_dir)
            .join("data")
            .join("sample-test-file.csv"),
        PathBuf::from(manifest_dir)
            .join("src")
            .join("data")
            .join("sample-test-file.csv"),
    ];

    let file_path = candidate_paths
        .into_iter()
        .find(|p| p.exists())
        .expect("Could not find sample-test-file.csv");

    let file = File::open(&file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers().unwrap().clone();

    let loinc_idx = headers
        .iter()
        .position(|h| h.to_lowercase().contains("loinc"))
        .unwrap_or(0);
    let unit_idx = headers
        .iter()
        .position(|h| h.to_lowercase().contains("unit"))
        .unwrap_or(1);

    let mut count = 0;

    for result in rdr.records() {
        let record = result.unwrap();
        let loinc = record.get(loinc_idx).unwrap_or_default();
        let unit = record.get(unit_idx).unwrap_or_default();

        let res = validator.validate_loinc_unit(loinc, unit);
        assert!(!res.unit_status.as_str().is_empty());
        count += 1;
    }

    assert!(count > 0);
}
