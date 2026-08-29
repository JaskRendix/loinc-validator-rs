use crate::validator::{LoincVldStatus, UnitVldStatus};

pub fn get_unit_note(status: UnitVldStatus) -> &'static str {
    match status {
        UnitVldStatus::VALID => "The unit is a valid UCUM unit.",
        UnitVldStatus::InvalidFixed => {
            "The unit is not a UCUM unit but there is a known mapping to a UCUM unit"
        }
        UnitVldStatus::InvalidUnknown => {
            "The unit is not a UCUM unit and there is no known mapping to a UCUM unit"
        }
        UnitVldStatus::MissingUnit => "The unit is not provided",
    }
}

pub fn get_loinc_note(status: Option<LoincVldStatus>) -> &'static str {
    match status {
        Some(LoincVldStatus::CORRECT) => "The LOINC mapping matches with the unit",
        Some(LoincVldStatus::INCORRECT) => "The LOINC mapping does not match with the unit",
        Some(LoincVldStatus::UNKNOWN) => "Unit information not available for the LOINC number",
        Some(LoincVldStatus::MissingLoinc) => "The LOINC number is not provided",
        None => "",
    }
}
