#[derive(Default, Debug, Clone)]
pub struct ValidationStats {
    pub valid_units: usize,
    pub invalid_fixed_units: usize,
    pub invalid_unknown_units: usize,
    pub missing_units: usize,
    pub correct_loinc: usize,
    pub incorrect_loinc: usize,
    pub unknown_loinc: usize,
    pub missing_loinc: usize,
}

impl ValidationStats {
    pub fn merge(&mut self, other: &ValidationStats) {
        self.valid_units += other.valid_units;
        self.invalid_fixed_units += other.invalid_fixed_units;
        self.invalid_unknown_units += other.invalid_unknown_units;
        self.missing_units += other.missing_units;
        self.correct_loinc += other.correct_loinc;
        self.incorrect_loinc += other.incorrect_loinc;
        self.unknown_loinc += other.unknown_loinc;
        self.missing_loinc += other.missing_loinc;
    }

    pub fn print_report(&self) {
        eprintln!("\n=== Validation Summary Statistics ===");
        eprintln!("Unit Status:");
        eprintln!("  VALID:          {}", self.valid_units);
        eprintln!("  INVALID_FIXED:  {}", self.invalid_fixed_units);
        eprintln!("  INVALID_UNKNOWN:{}", self.invalid_unknown_units);
        eprintln!("  MISSING_UNIT:   {}", self.missing_units);
        eprintln!("LOINC Status:");
        eprintln!("  CORRECT:        {}", self.correct_loinc);
        eprintln!("  INCORRECT:      {}", self.incorrect_loinc);
        eprintln!("  UNKNOWN:        {}", self.unknown_loinc);
        eprintln!("  MISSING_LOINC:  {}", self.missing_loinc);
        eprintln!("=====================================\n");
    }
}
