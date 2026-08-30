use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::loader::{LoadedData, load_all};
use crate::unit_analysis::{UnitAnalysis, analyze_unit};

const MAPPING_JSON: &str = include_str!("data/unit_to_ucum_mapping.json");
const LOINC_JSON: &str = include_str!("data/loinc_unit.json");

pub fn get_validator() -> Result<LoincValidator, Box<dyn std::error::Error>> {
    LoincValidator::new(LOINC_JSON, MAPPING_JSON)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum UnitVldStatus {
    VALID,
    #[serde(rename = "INVALID_FIXED")]
    InvalidFixed,
    #[serde(rename = "INVALID_UNKNOWN")]
    InvalidUnknown,
    #[serde(rename = "MISSING_UNIT")]
    MissingUnit,
}

impl UnitVldStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitVldStatus::VALID => "VALID",
            UnitVldStatus::InvalidFixed => "INVALID_FIXED",
            UnitVldStatus::InvalidUnknown => "INVALID_UNKNOWN",
            UnitVldStatus::MissingUnit => "MISSING_UNIT",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum LoincVldStatus {
    CORRECT,
    INCORRECT,
    UNKNOWN,
    #[serde(rename = "MISSING_LOINC")]
    MissingLoinc,
}

impl LoincVldStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoincVldStatus::CORRECT => "CORRECT",
            LoincVldStatus::INCORRECT => "INCORRECT",
            LoincVldStatus::UNKNOWN => "UNKNOWN",
            LoincVldStatus::MissingLoinc => "MISSING_LOINC",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidationResult {
    pub loinc: String,
    pub unit: String,
    pub unit_status: UnitVldStatus,
    pub loinc_status: Option<LoincVldStatus>,
    pub substituted_unit: Option<String>,
}

pub struct LoincValidator {
    loinc_to_units: FxHashMap<String, HashSet<String>>,
    units_to_ucum: FxHashMap<String, String>,
    strict: bool,
    loinc_version: Option<String>,
}

impl LoincValidator {
    pub fn new(loinc_json: &str, mapping_json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_strict(loinc_json, mapping_json, false)
    }

    pub fn new_with_strict(
        loinc_json: &str,
        mapping_json: &str,
        strict: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let LoadedData {
            loinc_to_units,
            units_to_ucum,
            loinc_version,
        } = load_all(loinc_json, mapping_json)?;

        Ok(Self {
            loinc_to_units,
            units_to_ucum,
            strict,
            loinc_version,
        })
    }

    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    pub fn loinc_version(&self) -> Option<&str> {
        self.loinc_version.as_deref()
    }

    fn normalize_loinc(&self, loinc: &str) -> String {
        loinc.trim().replace('\u{00A0}', "").to_string()
    }

    pub fn validate_loinc_unit(&self, loinc: &str, unit: &str) -> ValidationResult {
        let trimmed_loinc = self.normalize_loinc(loinc);
        let trimmed_unit = unit.trim();

        let analysis: UnitAnalysis = analyze_unit(trimmed_unit, self.strict, &self.units_to_ucum);

        if trimmed_loinc.is_empty() {
            let loinc_status = if self.strict {
                Some(LoincVldStatus::INCORRECT)
            } else {
                Some(LoincVldStatus::MissingLoinc)
            };

            return ValidationResult {
                loinc: trimmed_loinc.to_string(),
                unit: trimmed_unit.to_string(),
                unit_status: analysis.status,
                loinc_status,
                substituted_unit: analysis.substituted,
            };
        }

        let loinc_status = match self.loinc_to_units.get(trimmed_loinc.as_str()) {
            None => Some(LoincVldStatus::UNKNOWN),
            Some(units_set) => {
                if analysis.status == UnitVldStatus::InvalidUnknown {
                    if self.strict {
                        Some(LoincVldStatus::INCORRECT)
                    } else {
                        Some(LoincVldStatus::UNKNOWN)
                    }
                } else if units_set.contains(&analysis.active_unit) {
                    Some(LoincVldStatus::CORRECT)
                } else {
                    Some(LoincVldStatus::INCORRECT)
                }
            }
        };

        ValidationResult {
            loinc: trimmed_loinc.to_string(),
            unit: trimmed_unit.to_string(),
            unit_status: analysis.status,
            loinc_status,
            substituted_unit: analysis.substituted,
        }
    }
}
