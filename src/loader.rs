use rustc_hash::FxHashMap;
use serde_json::Value;
use std::collections::HashSet;

pub struct LoadedData {
    pub loinc_to_units: FxHashMap<String, HashSet<String>>,
    pub units_to_ucum: FxHashMap<String, String>,
    pub loinc_version: Option<String>,
}

pub struct LoincMap {
    pub loinc_to_units: FxHashMap<String, HashSet<String>>,
    pub loinc_version: Option<String>,
}

pub fn load_loinc_map(loinc_json: &str) -> Result<LoincMap, Box<dyn std::error::Error>> {
    let raw_loinc_map: FxHashMap<String, Value> = serde_json::from_str(loinc_json)?;

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

    Ok(LoincMap {
        loinc_to_units,
        loinc_version,
    })
}

pub fn load_unit_mapping(
    mapping_json: &str,
) -> Result<FxHashMap<String, String>, Box<dyn std::error::Error>> {
    let raw_mapping: FxHashMap<String, String> = serde_json::from_str(mapping_json)?;
    let mut units_to_ucum = FxHashMap::default();
    units_to_ucum.reserve(raw_mapping.len());

    for (k, v) in raw_mapping {
        units_to_ucum.insert(k.to_lowercase(), v.trim().to_lowercase());
    }

    Ok(units_to_ucum)
}

pub fn load_all(
    loinc_json: &str,
    mapping_json: &str,
) -> Result<LoadedData, Box<dyn std::error::Error>> {
    let loinc_map = load_loinc_map(loinc_json)?;
    let units_to_ucum = load_unit_mapping(mapping_json)?;

    Ok(LoadedData {
        loinc_to_units: loinc_map.loinc_to_units,
        units_to_ucum,
        loinc_version: loinc_map.loinc_version,
    })
}
