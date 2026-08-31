use rustc_hash::FxHashMap;

use crate::config::ValidatorConfig;
use crate::validator::UnitVldStatus;

pub struct UnitAnalysis {
    pub status: UnitVldStatus,
    pub substituted: Option<String>,
    pub active_unit: String,
    pub suggestion: Option<String>,
}

pub static SYNONYMS: &[(&str, &str)] = &[
    ("milligrams", "mg"),
    ("grams", "g"),
    ("liters", "l"),
    ("milliliters", "ml"),
    ("cells", "{cells}"),
    ("percent", "%"),
];

pub static LOINC_UNIT_SYNONYMS: &[(&str, &str)] = &[
    ("mg/dl", "mg/dL"),
    ("mmol/l", "mmol/L"),
    ("cells/ul", "cells/uL"),
];

fn is_valid_ucum(unit: &str) -> bool {
    !unit.is_empty() && ucum::validate(unit).is_ok()
}

fn lookup_substitution(unit: &str, units_to_ucum: &FxHashMap<String, String>) -> Option<String> {
    let lower = unit.to_lowercase();
    units_to_ucum.get(&lower).cloned()
}

fn canonicalize_ucum(unit: &str) -> String {
    unit.to_lowercase().replace(['.', ' '], "")
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

    for (i, row) in dp.iter_mut().enumerate().take(a.len() + 1) {
        row[0] = i;
    }

    for (j, cell) in dp[0].iter_mut().enumerate().take(b.len() + 1) {
        *cell = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            dp[i + 1][j + 1] = (dp[i][j + 1] + 1)
                .min(dp[i + 1][j] + 1)
                .min(dp[i][j] + cost);
        }
    }

    dp[a.len()][b.len()]
}

fn suggest_unit(unit: &str, units_to_ucum: &FxHashMap<String, String>) -> Option<String> {
    let mut best: Option<(String, usize)> = None;

    for key in units_to_ucum.keys() {
        let dist = levenshtein(unit, key);
        if dist <= 2 {
            match &best {
                None => best = Some((key.clone(), dist)),
                Some((_, best_dist)) if dist < *best_dist => best = Some((key.clone(), dist)),
                _ => {}
            }
        }
    }

    best.map(|(u, _)| u)
}

pub fn analyze_unit(
    trimmed_unit: &str,
    config: &ValidatorConfig,
    units_to_ucum: &FxHashMap<String, String>,
) -> UnitAnalysis {
    if trimmed_unit.trim().is_empty() {
        return UnitAnalysis {
            status: UnitVldStatus::MissingUnit,
            substituted: None,
            active_unit: if config.strict {
                "__MISSING__".to_string()
            } else {
                String::new()
            },
            suggestion: None,
        };
    }

    let mut unit = trimmed_unit.trim().replace('\u{00A0}', " ");
    unit = unit.replace('\t', " ");
    unit = unit.replace('\r', "");
    unit = unit.replace('\n', "");
    unit = unit.trim_end_matches(['.', ',', ';']).to_string();

    let is_bracketed = unit.starts_with('{') && unit.ends_with('}');
    let inner_raw = if is_bracketed {
        &unit[1..unit.len() - 1]
    } else {
        &unit
    };
    let inner = inner_raw.trim();
    let lower_inner = inner.to_lowercase();

    let mut normalized_inner = SYNONYMS
        .iter()
        .find_map(|(syn, canon)| {
            if lower_inner == *syn {
                Some((*canon).to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| inner.to_string());

    for (syn, canon) in LOINC_UNIT_SYNONYMS {
        if normalized_inner.to_lowercase() == *syn {
            normalized_inner = canon.to_string();
        }
    }

    if config.enable_canonicalization {
        normalized_inner = canonicalize_ucum(&normalized_inner);
    }

    if is_valid_ucum(&normalized_inner) {
        return UnitAnalysis {
            status: UnitVldStatus::VALID,
            substituted: None,
            active_unit: normalized_inner.trim().to_string(),
            suggestion: None,
        };
    }

    if config.strict {
        let suggestion = if config.enable_suggestions {
            suggest_unit(&normalized_inner, units_to_ucum)
        } else {
            None
        };

        return UnitAnalysis {
            status: UnitVldStatus::InvalidUnknown,
            substituted: None,
            active_unit: normalized_inner.trim().to_string(),
            suggestion,
        };
    }

    if config.allow_substitution {
        let mapped = lookup_substitution(&unit, units_to_ucum)
            .or_else(|| lookup_substitution(&lower_inner, units_to_ucum))
            .or_else(|| lookup_substitution(&normalized_inner, units_to_ucum));

        if let Some(mapped_val) = mapped {
            return UnitAnalysis {
                status: UnitVldStatus::InvalidFixed,
                substituted: Some(mapped_val.clone()),
                active_unit: mapped_val.trim().to_string(),
                suggestion: None,
            };
        }
    }

    let suggestion = if config.enable_suggestions {
        suggest_unit(&normalized_inner, units_to_ucum)
    } else {
        None
    };

    UnitAnalysis {
        status: UnitVldStatus::InvalidUnknown,
        substituted: None,
        active_unit: normalized_inner.trim().to_string(),
        suggestion,
    }
}
