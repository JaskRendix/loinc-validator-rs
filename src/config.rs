use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    pub strict: bool,
    pub allow_substitution: bool,
    pub enable_canonicalization: bool,
    pub enable_suggestions: bool,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            strict: false,
            allow_substitution: true,
            enable_canonicalization: false,
            enable_suggestions: false,
        }
    }
}
