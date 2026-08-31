use crate::config::ValidatorConfig;
use crate::validator::LoincValidator as CoreValidator;
use wasm_bindgen::prelude::*;

const MAPPING_JSON: &str = include_str!("data/unit_to_ucum_mapping.json");
const LOINC_JSON: &str = include_str!("data/loinc_unit.json");

#[wasm_bindgen]
pub struct WasmLoincValidator {
    validator: CoreValidator,
}

#[wasm_bindgen]
impl WasmLoincValidator {
    #[wasm_bindgen(constructor)]
    pub fn new(strict: bool) -> Result<WasmLoincValidator, JsValue> {
        let config = ValidatorConfig {
            strict,
            ..ValidatorConfig::default()
        };

        let validator = CoreValidator::new_with_config(LOINC_JSON, MAPPING_JSON, config)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(WasmLoincValidator { validator })
    }

    #[wasm_bindgen(js_name = validateLoincUnit)]
    pub fn validate_loinc_unit(&self, loinc: &str, unit: &str) -> Result<String, JsValue> {
        let result = self.validator.validate_loinc_unit(loinc, unit);
        serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = getLoincVersion)]
    pub fn loinc_version(&self) -> Option<String> {
        self.validator.loinc_version().map(|s| s.to_string())
    }
}
