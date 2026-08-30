use loinc_validator_rs::validator::{LoincVldStatus, UnitVldStatus, get_validator};

#[test]
fn test_synonym_mapping() {
    let validator = get_validator().unwrap();

    let cases = vec![
        (
            "18833-4",
            "milligrams",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "grams",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "percent",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            " milligrams ",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "\tmilliliters\t",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "milligrams\u{00A0}",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "18833-4",
            "\u{200B}grams\u{200B}",
            UnitVldStatus::InvalidUnknown,
            Some(LoincVldStatus::UNKNOWN),
        ),
        (
            "18833-4",
            "cells",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "26464-8",
            "percent",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }

    // strict mode
    const MAPPING_JSON: &str = include_str!("../src/data/unit_to_ucum_mapping.json");
    const LOINC_JSON: &str = include_str!("../src/data/loinc_unit.json");

    let strict_validator = loinc_validator_rs::validator::LoincValidator::new_with_strict(
        LOINC_JSON,
        MAPPING_JSON,
        true,
    )
    .unwrap();

    let strict_cases = vec![
        (
            "18833-4",
            "milligrams",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
        (
            "26464-8",
            "percent",
            UnitVldStatus::VALID,
            Some(LoincVldStatus::INCORRECT),
        ),
    ];

    for (loinc, unit, expected_unit_status, expected_loinc_status) in strict_cases {
        let res = strict_validator.validate_loinc_unit(loinc, unit);
        assert_eq!(res.unit_status, expected_unit_status);
        assert_eq!(res.loinc_status, expected_loinc_status);
    }
}
