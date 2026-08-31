use loinc_validator_rs::config::ValidatorConfig;
use loinc_validator_rs::unit_analysis::analyze_unit;
use loinc_validator_rs::validator::UnitVldStatus;
use rustc_hash::FxHashMap;

fn make_cfg() -> ValidatorConfig {
    ValidatorConfig {
        enable_canonicalization: true,
        enable_suggestions: true,
        allow_substitution: true,
        strict: false,
    }
}

fn empty_map() -> FxHashMap<String, String> {
    FxHashMap::default()
}

#[test]
fn test_canonicalization_cases() {
    let cfg = make_cfg();
    let map = empty_map();

    let cases = vec![
        // Basic punctuation canonicalization
        ("MG.", "mg"),
        ("KG ", "kg"),
        ("kg.", "kg"),
        ("mg.", "mg"),
        // Bracketed UCUM canonicalization
        ("{kg.}", "kg"),
        ("{ mg }", "mg"),
        // Mixed noise
        (" mg. ", "mg"),
        ("\tKG.\n", "kg"),
    ];

    for (input, expected_active) in cases {
        let analysis = analyze_unit(input, &cfg, &map);
        assert_eq!(analysis.active_unit, expected_active);
        assert_eq!(analysis.status.as_str(), "VALID");
    }
}

#[test]
fn test_loinc_unit_synonyms() {
    let cfg = make_cfg();
    let map = empty_map();

    let cases = vec![
        ("mg/dl", "mg/dl"),
        ("MG/DL", "mg/dl"), // uppercase → substitution
        ("mmol/l", "mmol/l"),
        ("MMOL/L", "mmol/l"), // uppercase → substitution
    ];

    for (input, expected_active) in cases {
        let analysis = analyze_unit(input, &cfg, &map);
        assert_eq!(analysis.active_unit, expected_active);
        assert_eq!(analysis.status.as_str(), "VALID");
    }
}

#[test]
fn test_suggestion_cases() {
    let mut cfg = make_cfg();
    cfg.enable_suggestions = true;

    // Minimal UCUM map for suggestions
    let mut map = FxHashMap::default();
    map.insert("mg".into(), "mg".into());
    map.insert("kg".into(), "kg".into());
    map.insert("g".into(), "g".into());

    let cases = vec![
        // "miligrams" → suggestion "mg"
        ("miligrams", None),
        // "kgg" → suggestion "kg"
        ("kgg", Some("kg")),
        // uppercase noise
        ("KGGK", Some("kg")),
        // near miss
        ("mgg", Some("mg")),
        // far miss → no suggestion
        ("zzzz", None),
    ];

    for (input, expected_suggestion) in cases {
        let analysis = analyze_unit(input, &cfg, &map);
        assert_eq!(analysis.suggestion.as_deref(), expected_suggestion);
    }
}

#[test]
fn test_edge_cases() {
    let cfg = make_cfg();
    let map = empty_map();

    let cases = vec![
        // Empty unit
        ("", UnitVldStatus::MissingUnit, ""),
        // Whitespace only
        ("   ", UnitVldStatus::MissingUnit, ""),
        // Bracketed empty
        ("{}", UnitVldStatus::InvalidUnknown, ""),
        // Bracketed garbage
        ("{foo}", UnitVldStatus::InvalidUnknown, "foo"),
        // Unicode mg symbol (㎎)
        ("㎎", UnitVldStatus::InvalidUnknown, "㎎"),
        // Numeric garbage
        ("123", UnitVldStatus::VALID, "123"),
    ];

    for (input, expected_status, expected_active) in cases {
        let analysis = analyze_unit(input, &cfg, &map);
        assert_eq!(analysis.status, expected_status);
        assert_eq!(analysis.active_unit, expected_active);
    }
}

#[test]
fn test_substitution_and_strict_behavior() {
    let cfg = ValidatorConfig {
        strict: true,
        allow_substitution: false,
        enable_canonicalization: false,
        enable_suggestions: false,
    };
    let map = empty_map();

    let analysis = analyze_unit("unknown_unit", &cfg, &map);
    assert_eq!(analysis.status, UnitVldStatus::InvalidUnknown);
    assert!(analysis.substituted.is_none());
}

#[test]
fn test_trailing_punctuation_edge_cases() {
    let cfg = make_cfg();
    let map = empty_map();

    let cases = vec![("mg,", "mg"), ("g;", "g"), ("ml.", "ml")];

    for (input, expected_active) in cases {
        let analysis = analyze_unit(input, &cfg, &map);
        assert_eq!(analysis.active_unit, expected_active);
        assert_eq!(analysis.status.as_str(), "VALID");
    }
}
