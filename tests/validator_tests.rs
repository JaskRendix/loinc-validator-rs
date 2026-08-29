use loinc_validator_rs::validator::{LoincVldStatus, UnitVldStatus, get_validator};
use std::fs::File;
use std::path::PathBuf;

#[test]
fn test_loinc_mapping_parity_extended() {
    let validator = get_validator().unwrap();

    let test_cases: Vec<(&str, &str, UnitVldStatus, Option<LoincVldStatus>)> = vec![
        // --- VALID UCUM ---
        (
            "18833-4",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "83167-7",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "99999-9",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            " kg ",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "KG",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "Kg",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        // --- VALID BRACE UCUM ---
        (
            "18833-4",
            "{kg}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "{ kg }",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        // --- INVALID BRACE ---
        (
            "18833-4",
            "{}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "{foo}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        // --- SUBSTITUTION UNITS ---
        (
            "64015-1",
            "/week",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "64015-1",
            "/WEEK",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "83167-7",
            "/week",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "99999-9",
            "/week",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::UNKNOWN),
        ),
        // --- INVALID UNKNOWN ---
        (
            "83167-7",
            "kggk",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "83167-7",
            " kggk ",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "83167-7",
            "KGGK",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "83167-7",
            "㎎",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ), // Unicode mg symbol
        // --- MISSING UNIT ---
        (
            "18833-4",
            "",
            UnitVldStatus::MissingUnit,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "   ",
            UnitVldStatus::MissingUnit,
            Some(LoincVldStatus::INCORRECT),
        ),
        // --- MISSING LOINC ---
        (
            "",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::MissingLoinc),
        ),
        (
            "   ",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::MissingLoinc),
        ),
        // --- UNKNOWN LOINC ---
        (
            "99999-9",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "99999-9",
            "/week",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "abc",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "#$%",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
        // --- PATHOLOGICAL UNITS ---
        (
            "18833-4",
            "@@@",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "123",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "mg!!",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in test_cases {
        let res = validator.validate_loinc_unit(loinc, unit);

        assert_eq!(
            res.unit_status, expected_unit_status,
            "Unit status mismatch for LOINC '{}' with unit '{}'",
            loinc, unit
        );

        assert_eq!(
            res.loinc_status, expected_loinc_status,
            "LOINC status mismatch for LOINC '{}' with unit '{}'",
            loinc, unit
        );
    }
}

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
        .expect("Could not find sample-test-file.csv in 'data/' or 'src/data/'");

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
    assert!(count > 0, "No records found in sample CSV file");
}

#[test]
fn test_loinc_mapping_parity_strict() {
    // Initialize validator with strict mode enabled (true)
    const MAPPING_JSON: &str = include_str!("../src/data/unit_to_ucum_mapping.json");
    const LOINC_JSON: &str = include_str!("../src/data/loinc_unit.json");
    let validator = loinc_validator_rs::validator::LoincValidator::new_with_strict(
        LOINC_JSON,
        MAPPING_JSON,
        true,
    )
    .unwrap();

    let strict_test_cases: Vec<(&str, &str, UnitVldStatus, Option<LoincVldStatus>)> = vec![
        // 1. Strict mode: valid UCUM
        (
            "18833-4",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        // 2. Strict mode: brace UCUM
        (
            "18833-4",
            "{kg}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        // 3. Strict mode: invalid brace
        (
            "18833-4",
            "{foo}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        // 4. Strict mode: missing unit
        (
            "18833-4",
            "",
            UnitVldStatus::MissingUnit,
            Some(LoincVldStatus::INCORRECT),
        ),
        // 5. Strict mode: missing LOINC
        (
            "",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        // 6. Strict mode: substitution forbidden
        (
            "64015-1",
            "/week",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        // 7. Strict mode: unknown unit
        (
            "83167-7",
            "kggk",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        // 8. Strict mode: unknown LOINC
        (
            "99999-9",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in strict_test_cases {
        let res = validator.validate_loinc_unit(loinc, unit);

        assert_eq!(
            res.unit_status, expected_unit_status,
            "[Strict] Unit status mismatch for LOINC '{}' with unit '{}'",
            loinc, unit
        );

        assert_eq!(
            res.loinc_status, expected_loinc_status,
            "[Strict] LOINC status mismatch for LOINC '{}' with unit '{}'",
            loinc, unit
        );
    }
}
