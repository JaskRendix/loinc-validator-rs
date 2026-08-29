# Loinc Mapping Validator (Rust Edition)

A high-performance Rust port of the official National Library of Medicine (NLM) [LOINC Mapping Validator](https://github.com/lhncbc/loinc-mapping-validator). This tool checks whether clinical record LOINC codes match their associated units, offering both a modular core validation library and a memory-efficient CLI for batch processing large datasets.

## The Modernization Story

Data engineering teams struggle to run legacy research scripts on massive healthcare datasets. This project transforms the original Node.js reference tool into a lightning-fast, zero-overhead Rust crate. By leveraging embedded JSON lookups via `include_str!`, streaming CSV parsers, and thread-pool parallelism via `rayon`, it scales across multi-core systems while preserving complete functional parity with the NLM standard.

---

## Crate Architecture

The project is structured as a dual library and binary workspace, allowing other Rust projects to embed the validation engine:

* **`lib.rs`**: Exposes reusable modules (`validator`, `cli`, `notes`, `output`, `stats`).
* **`main.rs`**: Provides the binary CLI frontend driving parallel chunk processing.
* **Tests**: Comprehensive integration and parity test suites (`tests/integration_tests.rs`) validating edge cases against the NLM behavior.

### Library Usage Example

```rust
use loinc_validator_rs::validator::LoincValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MAPPING_JSON: &str = include_str!("data/unit_to_ucum_mapping.json");
    const LOINC_JSON: &str = include_str!("data/loinc_unit.json");

    let validator = LoincValidator::new_with_strict(LOINC_JSON, MAPPING_JSON, false)?;
    let result = validator.validate_loinc_unit("18833-4", "kg");
    
    println!("Unit Status: {}", result.unit_status.as_str());
    Ok(())
}

```

---

## The Library API

### Core Functions

* **`validate_loinc_unit(loinc: &str, unit: &str)`**: Validates whether a given LOINC code and unit pair correspond.
* **Inputs**:
* `loinc`: The LOINC number to check.
* `unit`: The UCUM or custom unit string.


* **Outputs**: A result struct containing the input fields, status codes, and recommended UCUM substitutions (`substituted_unit`).

### Validation Status Codes

* **Unit Validation Status (`UnitVldStatus`)**:
* `VALID`: The unit is a valid UCUM unit.
* `InvalidFixed`: Not a UCUM unit, but a known mapping exists under `substituted_unit`.
* `InvalidUnknown`: Not a UCUM unit and no known mapping exists.
* `MissingUnit`: The unit input was left blank.


* **LOINC Mapping Status (`LoincVldStatus`)**:
* `CORRECT`: The LOINC code matches the unit.
* `INCORRECT`: The LOINC code does not match the unit.
* `UNKNOWN`: Unit information is unavailable for the given LOINC code.
* `MissingLoinc`: The LOINC number was left blank.



---

## The Command-Line Interface (CLI)

The CLI tool batch-validates multi-megabyte CSV files line-by-line, appending validation results and explanatory notes to your output schema.

### Usage Syntax

```bash
loinc-validator-rs -i <input-csv-file> -l <loinc-column-name> -u <unit-column-name> [-o <output-file>]

```

### Input Requirements

* Standard CSV file format (tested against Microsoft Excel comma-delimited layouts).
* Must include a header row containing the specified LOINC and unit column names.

### Output File Format

The generated CSV retains all original columns and appends 5 validation metadata fields:

* `LMV_UNIT_STATUS`: Validation code for the unit.
* `LMV_LOINC_STATUS`: Validation code for the LOINC mapping.
* `LMV_SUBSTITUTED_UNIT`: Suggested UCUM unit replacement if applicable.
* `LMV_UNIT_NOTE`: Explanatory description of the unit status.
* `LMV_LOINC_NOTE`: Explanatory description of the LOINC status.

---

## Technical Architecture

* **Fast Data Loading**: Embedded JSON datasets (`loinc_unit.json` and `unit_to_ucum_mapping.json`) are parsed at startup using `serde_json` into read-only hash maps.
* **Low Memory Footprint**: Uses the `csv` crate to stream rows in chunks, preventing out-of-memory errors on massive production files.
* **Parallel Processing**: Powered by `rayon` to distribute row validation across available CPU threads via custom `fold`/`reduce` pipelines.
* **Strict Parity**: Tested against native Rust integration suites verifying absolute behavior alignment with reference datasets.
