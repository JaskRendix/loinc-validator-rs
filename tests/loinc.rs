use loinc_validator_rs::config::ValidatorConfig;
use loinc_validator_rs::validator::{LoincValidator, LoincVldStatus, UnitVldStatus, get_validator};

#[test]
fn test_loinc_mapping_parity_extended() {
    let validator = get_validator().unwrap();

    let test_cases: Vec<(&str, &str, UnitVldStatus, Option<LoincVldStatus>)> = vec![
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
        ),
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
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_loinc_mapping_parity_strict() {
    const MAPPING_JSON: &str = include_str!("../src/data/unit_to_ucum_mapping.json");
    const LOINC_JSON: &str = include_str!("../src/data/loinc_unit.json");

    let validator = loinc_validator_rs::validator::LoincValidator::new_with_strict(
        LOINC_JSON,
        MAPPING_JSON,
        true,
    )
    .unwrap();

    let strict_test_cases = vec![
        (
            "18833-4",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "{kg}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "{foo}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "",
            UnitVldStatus::MissingUnit,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "64015-1",
            "/week",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "83167-7",
            "kggk",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "99999-9",
            "kg",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in strict_test_cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_unit_suggestions_and_canonicalization() {
    const MAPPING_JSON: &str = r#"{"my_custom_unit": "kg"}"#;
    const LOINC_JSON: &str = r#"{"18833-4": ["kg"]}"#;

    let config = ValidatorConfig {
        enable_canonicalization: true,
        allow_substitution: true,
        strict: false,
        ..Default::default()
    };

    let validator = LoincValidator::new_with_config(LOINC_JSON, MAPPING_JSON, config).unwrap();

    let res = validator.validate_loinc_unit("18833-4", "my_custom_unit");

    assert_eq!(res.unit_status, UnitVldStatus::InvalidFixed);
    assert!(res.substituted_unit.is_some());
}
