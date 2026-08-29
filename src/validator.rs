use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

struct UnitAnalysis {
    status: UnitVldStatus,
    substituted: Option<String>,
    active_unit: String,
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
        let raw_loinc_map: FxHashMap<String, serde_json::Value> = serde_json::from_str(loinc_json)?;

        let mut loinc_version = None;
        let mut loinc_to_units = FxHashMap::default();
        loinc_to_units.reserve(raw_loinc_map.len());

        for (k, v) in raw_loinc_map {
            if k == "version" {
                loinc_version = v.as_str().map(|s| s.to_string());
                continue;
            }
            if let Some(arr) = v.as_array() {
                let units: HashSet<String> = arr
                    .iter()
                    .filter_map(|val| val.as_str().map(|s| s.trim().to_string()))
                    .collect();
                loinc_to_units.insert(k, units);
            }
        }

        let raw_mapping: FxHashMap<String, String> = serde_json::from_str(mapping_json)?;
        let mut units_to_ucum = FxHashMap::default();
        units_to_ucum.reserve(raw_mapping.len());
        for (k, v) in raw_mapping {
            units_to_ucum.insert(k.to_lowercase(), v.trim().to_lowercase());
        }

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

    fn analyze_unit(&self, trimmed_unit: &str) -> UnitAnalysis {
        if trimmed_unit.is_empty() {
            return UnitAnalysis {
                status: UnitVldStatus::MissingUnit,
                substituted: None,
                active_unit: if self.strict {
                    "__MISSING__".to_string()
                } else {
                    String::new()
                },
            };
        }

        let is_bracketed = trimmed_unit.starts_with('{') && trimmed_unit.ends_with('}');
        let inner_raw = if is_bracketed {
            &trimmed_unit[1..trimmed_unit.len() - 1]
        } else {
            trimmed_unit
        };
        let inner = inner_raw.trim();

        if !inner.is_empty() && ucum::validate(inner).is_ok() {
            return UnitAnalysis {
                status: UnitVldStatus::VALID,
                substituted: None,
                active_unit: inner.to_string(),
            };
        }

        if self.strict {
            return UnitAnalysis {
                status: UnitVldStatus::InvalidUnknown,
                substituted: None,
                active_unit: inner.to_string(),
            };
        }

        let lower_unit = trimmed_unit.to_lowercase();
        let lower_inner = inner.to_lowercase();

        let mapped = self
            .units_to_ucum
            .get(&lower_unit)
            .or_else(|| self.units_to_ucum.get(&lower_inner));

        if let Some(mapped_val) = mapped {
            UnitAnalysis {
                status: UnitVldStatus::InvalidFixed,
                substituted: Some(mapped_val.clone()),
                active_unit: mapped_val.clone(),
            }
        } else {
            UnitAnalysis {
                status: UnitVldStatus::InvalidUnknown,
                substituted: None,
                active_unit: inner.to_string(),
            }
        }
    }

    pub fn validate_loinc_unit(&self, loinc: &str, unit: &str) -> ValidationResult {
        let trimmed_loinc = loinc.trim();
        let trimmed_unit = unit.trim();

        let analysis = self.analyze_unit(trimmed_unit);

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

        let loinc_status = match self.loinc_to_units.get(trimmed_loinc) {
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
