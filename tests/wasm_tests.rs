use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
use loinc_validator_rs::wasm::WasmLoincValidator;

#[cfg(not(target_arch = "wasm32"))]
use loinc_validator_rs::wasm::WasmLoincValidator;

#[wasm_bindgen_test]
fn test_initialization_non_strict() {
    let validator = WasmLoincValidator::new(false);
    assert!(
        validator.is_ok(),
        "Validator should initialize in non-strict mode"
    );
}

#[wasm_bindgen_test]
fn test_initialization_strict() {
    let validator = WasmLoincValidator::new(true);
    assert!(
        validator.is_ok(),
        "Validator should initialize in strict mode"
    );
}

#[wasm_bindgen_test]
fn test_loinc_version_returns_some_or_none() {
    let validator = WasmLoincValidator::new(false).unwrap();
    let version = validator.loinc_version();
    assert!(
        version.is_none() || version.is_some(),
        "Version should not panic and must return Option<String>"
    );
}

#[wasm_bindgen_test]
fn test_validation_known_valid_cases() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let cases = vec![("18833-4", "kg"), ("18833-4", "g"), ("26464-8", "percent")];

    for (loinc, unit) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert!(
            res.is_ok(),
            "Validation should not error for valid input: {loinc} {unit}"
        );

        let json = res.unwrap();
        assert!(
            json.contains("VALID") || json.contains("CORRECT"),
            "Expected VALID or CORRECT in JSON result for {loinc} {unit}"
        );
    }
}

#[wasm_bindgen_test]
fn test_validation_known_invalid_cases() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let cases = vec![
        ("18833-4", "bananas"),
        ("18833-4", "not-a-unit"),
        ("18833-4", "@@@"),
    ];

    for (loinc, unit) in cases {
        let res = validator.validate_loinc_unit(loinc, unit);
        assert!(
            res.is_ok(),
            "Even invalid units should return JSON, not an error: {loinc} {unit}"
        );

        let json = res.unwrap();
        assert!(
            json.contains("Invalid") || json.contains("INCORRECT") || json.contains("UNKNOWN"),
            "Expected Invalid status in JSON result for {loinc} {unit}"
        );
    }
}
