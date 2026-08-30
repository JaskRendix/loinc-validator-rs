use rustc_hash::FxHashMap;

use crate::config::ValidatorConfig;
use crate::validator::UnitVldStatus;

pub struct UnitAnalysis {
    pub status: UnitVldStatus,
    pub substituted: Option<String>,
    pub active_unit: String,
}

/// Simple synonym table for common non‑UCUM unit labels.
pub static SYNONYMS: &[(&str, &str)] = &[
    ("milligrams", "mg"),
    ("grams", "g"),
    ("liters", "l"),
    ("milliliters", "ml"),
    ("cells", "{cells}"),
    ("percent", "%"),
];

fn is_valid_ucum(unit: &str) -> bool {
    !unit.is_empty() && ucum::validate(unit).is_ok()
}

fn lookup_substitution(unit: &str, units_to_ucum: &FxHashMap<String, String>) -> Option<String> {
    let lower = unit.to_lowercase();
    units_to_ucum.get(&lower).cloned()
}

pub fn analyze_unit(
    trimmed_unit: &str,
    config: &ValidatorConfig,
    units_to_ucum: &FxHashMap<String, String>,
) -> UnitAnalysis {
    // 1. Empty unit
    if trimmed_unit.trim().is_empty() {
        return UnitAnalysis {
            status: UnitVldStatus::MissingUnit,
            substituted: None,
            active_unit: if config.strict {
                "__MISSING__".to_string()
            } else {
                String::new()
            },
        };
    }

    // 2. Normalize Unicode whitespace + invisible chars
    let mut unit = trimmed_unit.trim().replace('\u{00A0}', " "); // non-breaking space
    unit = unit.replace('\t', " ");
    unit = unit.replace('\r', "");
    unit = unit.replace('\n', "");

    // 3. Strip trailing punctuation (common Excel/ETL artifact)
    unit = unit.trim_end_matches(['.', ',', ';']).to_string();

    // 4. Handle bracketed UCUM units: {cells} → cells
    let is_bracketed = unit.starts_with('{') && unit.ends_with('}');
    let inner_raw = if is_bracketed {
        &unit[1..unit.len() - 1]
    } else {
        &unit
    };
    let inner = inner_raw.trim();

    // 5. Canonical lowercase version
    let lower_inner = inner.to_lowercase();

    // 6. Synonym mapping (expandable)
    let normalized_inner = SYNONYMS
        .iter()
        .find_map(|(syn, canon)| {
            if lower_inner == *syn {
                Some((*canon).to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| inner.to_string());

    // 7. UCUM validation
    if is_valid_ucum(&normalized_inner) {
        return UnitAnalysis {
            status: UnitVldStatus::VALID,
            substituted: None,
            active_unit: normalized_inner.trim().to_string(),
        };
    }

    // 8. Strict mode: no fallback, no substitution
    if config.strict {
        return UnitAnalysis {
            status: UnitVldStatus::InvalidUnknown,
            substituted: None,
            active_unit: normalized_inner.trim().to_string(),
        };
    }

    // 9. Non-strict mode: try substitution map
    if config.allow_substitution {
        let mapped = lookup_substitution(&unit, units_to_ucum)
            .or_else(|| lookup_substitution(&lower_inner, units_to_ucum))
            .or_else(|| lookup_substitution(&normalized_inner, units_to_ucum));

        if let Some(mapped_val) = mapped {
            return UnitAnalysis {
                status: UnitVldStatus::InvalidFixed,
                substituted: Some(mapped_val.clone()),
                active_unit: mapped_val.trim().to_string(),
            };
        }
    }

    // 10. Unknown unit
    UnitAnalysis {
        status: UnitVldStatus::InvalidUnknown,
        substituted: None,
        active_unit: normalized_inner.trim().to_string(),
    }
}
