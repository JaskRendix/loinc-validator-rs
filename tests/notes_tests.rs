use loinc_validator_rs::notes::{get_loinc_note, get_unit_note};
use loinc_validator_rs::validator::{LoincVldStatus, UnitVldStatus};

#[test]
fn test_get_unit_note_valid() {
    let note = get_unit_note(UnitVldStatus::VALID);
    assert_eq!(note, "The unit is a valid UCUM unit.");
}

#[test]
fn test_get_unit_note_invalid_fixed() {
    let note = get_unit_note(UnitVldStatus::InvalidFixed);
    assert_eq!(
        note,
        "The unit is not a UCUM unit but there is a known mapping to a UCUM unit"
    );
}

#[test]
fn test_get_unit_note_invalid_unknown() {
    let note = get_unit_note(UnitVldStatus::InvalidUnknown);
    assert_eq!(
        note,
        "The unit is not a UCUM unit and there is no known mapping to a UCUM unit"
    );
}

#[test]
fn test_get_unit_note_missing_unit() {
    let note = get_unit_note(UnitVldStatus::MissingUnit);
    assert_eq!(note, "The unit is not provided");
}

#[test]
fn test_get_loinc_note_correct() {
    let note = get_loinc_note(Some(LoincVldStatus::CORRECT));
    assert_eq!(note, "The LOINC mapping matches with the unit");
}

#[test]
fn test_get_loinc_note_incorrect() {
    let note = get_loinc_note(Some(LoincVldStatus::INCORRECT));
    assert_eq!(note, "The LOINC mapping does not match with the unit");
}

#[test]
fn test_get_loinc_note_unknown() {
    let note = get_loinc_note(Some(LoincVldStatus::UNKNOWN));
    assert_eq!(note, "Unit information not available for the LOINC number");
}

#[test]
fn test_get_loinc_note_missing_loinc() {
    let note = get_loinc_note(Some(LoincVldStatus::MissingLoinc));
    assert_eq!(note, "The LOINC number is not provided");
}

#[test]
fn test_get_loinc_note_none() {
    let note = get_loinc_note(None);
    assert_eq!(note, "");
}

#[test]
fn test_notes_do_not_panic() {
    // Just ensure no panics for any variant
    let _ = get_unit_note(UnitVldStatus::VALID);
    let _ = get_unit_note(UnitVldStatus::InvalidFixed);
    let _ = get_unit_note(UnitVldStatus::InvalidUnknown);
    let _ = get_unit_note(UnitVldStatus::MissingUnit);

    let _ = get_loinc_note(Some(LoincVldStatus::CORRECT));
    let _ = get_loinc_note(Some(LoincVldStatus::INCORRECT));
    let _ = get_loinc_note(Some(LoincVldStatus::UNKNOWN));
    let _ = get_loinc_note(Some(LoincVldStatus::MissingLoinc));
    let _ = get_loinc_note(None);
}
