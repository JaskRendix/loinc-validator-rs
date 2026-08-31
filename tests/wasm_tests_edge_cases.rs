use wasm_bindgen_test::*;

use loinc_validator_rs::wasm::WasmLoincValidator;

#[wasm_bindgen_test]
fn test_empty_strings() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let res = validator.validate_loinc_unit("", "");
    assert!(res.is_ok(), "Empty strings should still produce JSON");

    let json = res.unwrap();
    // Match the actual status strings defined in UnitVldStatus / LoincVldStatus
    assert!(
        json.contains("Missing")
            || json.contains("INVALID")
            || json.contains("INCORRECT")
            || json.contains("UNKNOWN"),
        "Empty input should reflect missing or invalid status"
    );
}

#[wasm_bindgen_test]
fn test_whitespace_inputs() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let res = validator.validate_loinc_unit("   ", "   ");
    assert!(res.is_ok(), "Whitespace should not crash");

    let json = res.unwrap();
    assert!(
        json.contains("Missing")
            || json.contains("INVALID")
            || json.contains("INCORRECT")
            || json.contains("UNKNOWN"),
        "Whitespace should reflect missing or invalid status"
    );
}

#[wasm_bindgen_test]
fn test_non_utf8_like_sequences() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let weird = "\u{FFFD}\u{FFFD}";
    let res = validator.validate_loinc_unit(weird, weird);

    assert!(res.is_ok(), "Non-UTF8 replacement chars should not crash");

    let json = res.unwrap();
    assert!(
        json.contains("Invalid") || json.contains("UNKNOWN") || json.contains("INCORRECT"),
        "Weird unicode should be invalid"
    );
}

#[wasm_bindgen_test]
fn test_extremely_long_inputs() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let long_loinc = "12345-6".repeat(500);
    let long_unit = "kg".repeat(500);

    let res = validator.validate_loinc_unit(&long_loinc, &long_unit);
    assert!(res.is_ok(), "Long inputs should not crash");

    let json = res.unwrap();
    assert!(
        json.contains("Invalid")
            || json.contains("UNKNOWN")
            || json.contains("VALID")
            || json.contains("INCORRECT"),
        "Long inputs must still produce JSON"
    );
}

#[wasm_bindgen_test]
fn test_strict_mode_behavior() {
    let validator = WasmLoincValidator::new(true).unwrap();

    let res = validator.validate_loinc_unit("18833-4", "KG");
    assert!(res.is_ok(), "Strict mode should still return JSON");

    let json = res.unwrap();
    assert!(
        json.contains("Invalid") || json.contains("INCORRECT") || json.contains("UNKNOWN"),
        "Strict mode should flag non-canonical casing as invalid/incorrect"
    );
}

#[wasm_bindgen_test]
fn test_invalid_json_handling() {
    let validator = WasmLoincValidator::new(false).unwrap();

    let res = validator.validate_loinc_unit("NOT-A-LOINC", "kg");
    assert!(res.is_ok(), "Invalid LOINC should not crash");

    let json = res.unwrap();
    assert!(
        json.contains("UNKNOWN") || json.contains("MissingLoinc"),
        "Invalid LOINC should produce an UNKNOWN or missing status result"
    );
}
