use loinc_validator_rs::validator::{LoincVldStatus, UnitVldStatus, get_validator};

#[test]
fn test_unicode_unit_normalization() {
    let validator = get_validator().unwrap();

    let cases = vec![
        (
            "18833-4",
            "mg\u{00A0}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "\u{00A0}kg\u{00A0}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg\t",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg\r\n",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "㎎",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg\u{200B}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "\u{200B}\u{00A0}kg\u{200B}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "k\u{0307}g",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_punctuation_stripping() {
    let validator = get_validator().unwrap();

    let cases = vec![
        (
            "18833-4",
            "kg.",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg,",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg;",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg...",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg,,",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg;;;",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "kg.,;",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            " kg. ",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "\tkg,\n",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "64015-1",
            "/week.",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "64015-1",
            "/week;",
            UnitVldStatus::InvalidFixed,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "26464-8",
            "percent.",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "26464-8",
            "milligrams;",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "@@@.",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg@@@.",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "mg!!.",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "{kg}.",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
        (
            "18833-4",
            "{ kg };",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::CORRECT),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_pathological_units() {
    let validator = get_validator().unwrap();

    let cases = vec![
        (
            "18833-4",
            "@@@",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "###",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "$$",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg123",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "123kg",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "mg!!",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg!!",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg@@",
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
            "999",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "\u{2603}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "\u{1F4A9}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg\u{1F4A9}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "{}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "{ }",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "{kg",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg\u{200B}\u{200B}@@@",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_embedded_control_characters() {
    let validator = get_validator().unwrap();

    let cases = vec![
        (
            "18833-4",
            "kg\0",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "mg\u{0007}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "kg\u{00AD}mg", // Soft hyphen
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}

#[test]
fn test_extremely_long_inputs() {
    let validator = get_validator().unwrap();
    let long_unit = "kg".repeat(1000);

    let res = validator.validate_loinc_unit("18833-4", &long_unit);
    assert_eq!(res.unit_status, UnitVldStatus::InvalidUnknown);
    assert_eq!(res.loinc_status, Some(LoincVldStatus::UNKNOWN));
}
