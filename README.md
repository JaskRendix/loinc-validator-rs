# **Loinc Mapping Validator (Rust Edition)**  
A fast, zero‑overhead Rust port of the official National Library of Medicine (NLM) **LOINC Mapping Validator**.  
It checks whether clinical LOINC codes match their units — but with Rust‑level performance, streaming CSV handling, and a clean modular API.

This crate ships both:

- a **library** you can embed anywhere  
- a **CLI** that can chew through multi‑million‑row CSVs without breaking a sweat  

All while staying fully aligned with the NLM reference behavior.

---

## **Why This Exists**

Healthcare teams often inherit old validation scripts that choke on real‑world data sizes.  
The original validator was a Node.js research tool — good for demos, not for production pipelines.

This Rust edition fixes that:

- everything is embedded at compile time (`include_str!`)  
- UCUM + LOINC validation runs at native speed  
- CSV rows stream in constant memory  
- Rayon parallelism scales across all cores  
- strict mode matches NLM’s exact behavior  

Same logic.  
Modern engine.

---

## **Crate Layout**

The project is a dual library/binary workspace:

- **`lib.rs`** — exposes reusable modules:  
  `validator`, `cli`, `notes`, `output`, `stats`
- **`main.rs`** — the CLI frontend  
- **`tests/`** — integration + parity tests matching NLM behavior

Everything is designed so you can either:

- call the validator directly from Rust  
- or run the CLI on huge CSVs

---

## **Library Usage**

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

## **What the Library Actually Does**

### **Core Validation**
`validate_loinc_unit(loinc, unit)` returns:

- the normalized unit  
- UCUM validation status  
- LOINC correctness  
- substituted UCUM unit (if a synonym matched)  
- human‑readable notes explaining everything  

### **Unit Validation Status**
- **VALID** — UCUM says yes  
- **InvalidFixed** — not UCUM, but a known synonym maps to a valid UCUM unit  
- **InvalidUnknown** — not UCUM, no synonym  
- **MissingUnit** — empty input

### **LOINC Mapping Status**
- **CORRECT** — LOINC allows this UCUM unit  
- **INCORRECT** — LOINC exists but doesn’t allow it  
- **UNKNOWN** — LOINC exists but has no unit info  
- **MissingLoinc** — empty input

---

## **Implemented Features (Important)**

These were missing from your README before — now they’re included.

### **UCUM Validation**
Full UCUM grammar support, including:

- `{cells}` arbitrary units  
- `%`  
- `mg`, `g`, `mL`, etc.

### **Synonym Substitution Layer**
Maps common clinical units to UCUM:

- `"milligrams"` → `"mg"`  
- `"grams"` → `"g"`  
- `"milliliters"` → `"mL"`  
- `"cells"` → `"{cells}"`

### **Advanced Normalization**
Handles real‑world mess:

- trimming  
- tabs  
- NBSP (`\u{00A0}`)  
- lowercasing  
- bracket stripping  
- preserves ZWSP (`\u{200B}`) to avoid false positives

### **Strict Mode**
Exact UCUM compliance:

- no heuristic substitutions  
- no fallback normalization  
- matches NLM strict behavior

### **Embedded JSON Datasets**
LOINC + UCUM mapping data is compiled in:

- zero runtime I/O  
- instant startup  
- portable binary

### **Detailed Notes System**
Every validation result includes:

- a unit note  
- a LOINC note  
- explanations for each status

### **Parity Test Suite**
Integration tests ensure:

- UCUM parity  
- synonym parity  
- strict‑mode parity  
- LOINC correctness parity  
- edge‑case handling

---

## **CLI Usage**

```bash
loinc-validator-rs \
  -i input.csv \
  -l LOINC_COLUMN \
  -u UNIT_COLUMN \
  -o output.csv
```

### **Input Requirements**
- CSV with header row  
- LOINC + unit columns present  
- Excel‑compatible comma‑delimited format

### **Output Columns**
The CLI appends:

- `LMV_UNIT_STATUS`  
- `LMV_LOINC_STATUS`  
- `LMV_SUBSTITUTED_UNIT`  
- `LMV_UNIT_NOTE`  
- `LMV_LOINC_NOTE`

---

## **Technical Architecture**

### **Fast Data Loading**
JSON datasets are embedded and parsed once at startup.

### **Streaming CSV Processing**
Rows are streamed in chunks — no full‑file loading.

### **Parallel Validation**
Rayon distributes row validation across all CPU cores.

### **Strict Parity**
Behavior matches the NLM reference validator, including:

- UCUM edge cases  
- synonym behavior  
- strict mode  
- LOINC correctness rules
