# **Loinc Mapping Validator (Rust Edition)**

Rust port of the National Library of Medicine (NLM) [LOINC Mapping Validator](https://github.com/lhncbc/loinc-mapping-validator). Checks whether a LOINC code and a unit match using UCUM and LOINC reference data. Works as a library and a CLI.

## **Why This Exists**

The original validator was a Node.js prototype. This version handles real datasets and deterministic behavior:

* UCUM + LOINC data embedded at compile time
* streaming CSV and JSONL
* parallel row validation
* strict mode matching NLM behavior

---

## **Layout**

* `src/lib.rs` — library entry point
* `src/main.rs` — CLI binary
* `src/config.rs` — `ValidatorConfig` struct definitions
* `src/loader.rs` — JSON asset loader module
* `src/validator.rs` — core validation engine
* `src/unit_analysis.rs` — unit parsing, canonicalization, and suggestion logic
* `src/output.rs` — CSV and JSONL output handlers
* `src/stats.rs` — batch validation statistics tracking
* `tests/` — parity and edge-case tests

---

## **Library Usage**

Basic validation:

```rust
use loinc_validator_rs::validator::LoincValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MAP: &str = include_str!("data/unit_to_ucum_mapping.json");
    const LOINC: &str = include_str!("data/loinc_unit.json");

    let v = LoincValidator::new_with_strict(LOINC, MAP, false)?;
    let r = v.validate_loinc_unit("18833-4", "kg");

    println!("{}", r.unit_status.as_str());
    Ok(())
}
```

Custom configuration:

```rust
use loinc_validator_rs::validator::LoincValidator;
use loinc_validator_rs::config::ValidatorConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MAP: &str = include_str!("data/unit_to_ucum_mapping.json");
    const LOINC: &str = include_str!("data/loinc_unit.json");

    let config = ValidatorConfig {
        enable_canonicalization: true,
        enable_suggestions: true,
        allow_substitution: true,
        strict: false,
    };

    let v = LoincValidator::new_with_config(LOINC, MAP, config)?;
    let r = v.validate_loinc_unit("18833-4", "MG");

    println!("{:?}", r.substituted_unit);
    Ok(())
}
```

---

## **Return Values**

`validate_loinc_unit(loinc, unit)` returns a `ValidationResult` containing:

* normalized unit
* unit status
* LOINC status
* substituted unit (if a synonym or fix matched)
* notes for unit and LOINC validation steps

### **Unit Status**

* `VALID`
* `InvalidFixed`
* `InvalidUnknown`
* `MissingUnit`

### **LOINC Status**

* `CORRECT`
* `INCORRECT`
* `UNKNOWN`
* `MissingLoinc`

---

## **Features**

* **UCUM Validation**: Evaluates UCUM grammar expressions (`{cells}`, `%`, `mg`, `g`, `mL`).
* **Synonyms**: Maps common terms to UCUM equivalents (`"milligrams"` -> `"mg"`, `"grams"` -> `"g"`, `"cells"` -> `"{cells}"`).
* **Normalization**: Handles whitespace, tabs, non-breaking spaces, case variants, bracket stripping, and preserves zero-width spaces.
* **Strict Mode**: Enforces exact UCUM compliance without heuristic fallbacks.
* **Embedded Data**: Compiles LOINC and UCUM JSON data directly into the binary.
* **JSONL Streaming**: Supports newline-delimited JSON output streams via `--format jsonl`.
* **WASM Target**: Compiles to WebAssembly using `wasm-bindgen` and `cdylib` feature configurations.
* **Batch Statistics Export**: Tracks and exports execution metrics to JSON using `--stats-output`.
* **Closest-Unit Suggestions**: Uses Levenshtein distance calculations to suggest alternatives for unknown inputs.

---

## **CLI**

```bash
loinc-validator-rs \
  -i input.csv \
  -l LOINC_COLUMN \
  -u UNIT_COLUMN \
  -o output.csv \
  --format jsonl \
  --stats-output stats.json
```

### **Generated Output Columns**

* `LMV_UNIT_STATUS`
* `LMV_LOINC_STATUS`
* `LMV_SUBSTITUTED_UNIT`
* `LMV_UNIT_NOTE`
* `LMV_LOINC_NOTE`

---

## **Architecture**

* Reference datasets parse once on initialization.
* Streaming input processes row chunks.
* Parallel processing validates rows concurrently.
* Implements NLM validation logic parity.
