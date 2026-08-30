# **Loinc Mapping Validator (Rust Edition)**  
Rust port of the National Library of Medicine (NLM) [LOINC Mapping Validator](https://github.com/lhncbc/loinc-mapping-validator).  
Checks whether a LOINC code and a unit match, using UCUM + LOINC reference data.  
Provides both a library and a CLI.

---

## **Why This Exists**
The original validator was a small Node.js prototype.  
This version is built for real datasets and deterministic behavior:

- UCUM + LOINC data embedded at compile time  
- streaming CSV  
- parallel row validation  
- strict mode identical to NLM behavior

---

## **Layout**
- `lib.rs` — core modules (`validator`, `unit_analysis`, `notes`, `output`, `stats`)  
- `main.rs` — CLI  
- `tests/` — UCUM + LOINC parity tests

Use it as a library or run the CLI on large CSVs.

---

## **Library Example**

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

---

## **What It Returns**
`validate_loinc_unit(loinc, unit)` gives:

- normalized unit  
- UCUM status  
- LOINC status  
- substituted UCUM unit (if a synonym matched)  
- notes explaining each step

### **Unit Status**
- **VALID**  
- **InvalidFixed**  
- **InvalidUnknown**  
- **MissingUnit**

### **LOINC Status**
- **CORRECT**  
- **INCORRECT**  
- **UNKNOWN**  
- **MissingLoinc**

---

## **Features**

### **UCUM Validation**
Full UCUM grammar: `{cells}`, `%`, `mg`, `g`, `mL`, etc.

### **Synonyms**
Maps common clinical units:

- `"milligrams"` → `"mg"`  
- `"grams"` → `"g"`  
- `"milliliters"` → `"mL"`  
- `"cells"` → `"{cells}"`

### **Normalization**
Handles messy input:

- trim  
- tabs  
- NBSP  
- lowercase  
- bracket stripping  
- preserves ZWSP

### **Strict Mode**
Exact UCUM compliance.  
No heuristics.  
Matches NLM behavior.

### **Embedded Data**
LOINC + UCUM mapping JSON included at compile time.

### **Notes**
Each result includes unit + LOINC notes.

### **Parity Tests**
Covers UCUM behavior, synonyms, strict mode, LOINC correctness, edge cases.

---

## **CLI**

```bash
loinc-validator-rs \
  -i input.csv \
  -l LOINC_COLUMN \
  -u UNIT_COLUMN \
  -o output.csv
```

### **Input**
- CSV with header  
- LOINC + unit columns

### **Output Columns**
- `LMV_UNIT_STATUS`  
- `LMV_LOINC_STATUS`  
- `LMV_SUBSTITUTED_UNIT`  
- `LMV_UNIT_NOTE`  
- `LMV_LOINC_NOTE`

---

## **Architecture**
- UCUM + LOINC data parsed once  
- streaming CSV  
- parallel row validation  
- strict parity with NLM reference logic
