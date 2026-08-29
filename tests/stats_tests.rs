use loinc_validator_rs::stats::ValidationStats;

#[test]
fn merge_empty_into_empty() {
    let mut a = ValidationStats::default();
    let b = ValidationStats::default();

    a.merge(&b);

    assert_eq!(a.valid_units, 0);
    assert_eq!(a.invalid_fixed_units, 0);
    assert_eq!(a.invalid_unknown_units, 0);
    assert_eq!(a.missing_units, 0);
    assert_eq!(a.correct_loinc, 0);
    assert_eq!(a.incorrect_loinc, 0);
    assert_eq!(a.unknown_loinc, 0);
    assert_eq!(a.missing_loinc, 0);
}

#[test]
fn merge_non_empty_into_empty() {
    let mut a = ValidationStats::default();
    let b = ValidationStats {
        valid_units: 5,
        invalid_fixed_units: 2,
        invalid_unknown_units: 3,
        missing_units: 1,
        correct_loinc: 7,
        incorrect_loinc: 4,
        unknown_loinc: 6,
        missing_loinc: 8,
    };

    a.merge(&b);

    assert_eq!(a.valid_units, 5);
    assert_eq!(a.invalid_fixed_units, 2);
    assert_eq!(a.invalid_unknown_units, 3);
    assert_eq!(a.missing_units, 1);
    assert_eq!(a.correct_loinc, 7);
    assert_eq!(a.incorrect_loinc, 4);
    assert_eq!(a.unknown_loinc, 6);
    assert_eq!(a.missing_loinc, 8);
}

#[test]
fn merge_empty_into_non_empty() {
    let mut a = ValidationStats {
        valid_units: 10,
        invalid_fixed_units: 1,
        invalid_unknown_units: 2,
        missing_units: 3,
        correct_loinc: 4,
        incorrect_loinc: 5,
        unknown_loinc: 6,
        missing_loinc: 7,
    };
    let b = ValidationStats::default();

    a.merge(&b);

    assert_eq!(a.valid_units, 10);
    assert_eq!(a.invalid_fixed_units, 1);
    assert_eq!(a.invalid_unknown_units, 2);
    assert_eq!(a.missing_units, 3);
    assert_eq!(a.correct_loinc, 4);
    assert_eq!(a.incorrect_loinc, 5);
    assert_eq!(a.unknown_loinc, 6);
    assert_eq!(a.missing_loinc, 7);
}

#[test]
fn merge_two_non_empty_stats() {
    let mut a = ValidationStats {
        valid_units: 1,
        invalid_fixed_units: 2,
        invalid_unknown_units: 3,
        missing_units: 4,
        correct_loinc: 5,
        incorrect_loinc: 6,
        unknown_loinc: 7,
        missing_loinc: 8,
    };

    let b = ValidationStats {
        valid_units: 10,
        invalid_fixed_units: 20,
        invalid_unknown_units: 30,
        missing_units: 40,
        correct_loinc: 50,
        incorrect_loinc: 60,
        unknown_loinc: 70,
        missing_loinc: 80,
    };

    a.merge(&b);

    assert_eq!(a.valid_units, 11);
    assert_eq!(a.invalid_fixed_units, 22);
    assert_eq!(a.invalid_unknown_units, 33);
    assert_eq!(a.missing_units, 44);
    assert_eq!(a.correct_loinc, 55);
    assert_eq!(a.incorrect_loinc, 66);
    assert_eq!(a.unknown_loinc, 77);
    assert_eq!(a.missing_loinc, 88);
}

#[test]
fn merge_is_associative() {
    let a = ValidationStats {
        valid_units: 1,
        ..ValidationStats::default()
    };
    let b = ValidationStats {
        valid_units: 2,
        ..ValidationStats::default()
    };
    let c = ValidationStats {
        valid_units: 3,
        ..ValidationStats::default()
    };

    let mut left = ValidationStats::default();
    left.merge(&a);
    left.merge(&b);
    left.merge(&c);

    let mut right = ValidationStats::default();
    right.merge(&b);
    right.merge(&c);
    right.merge(&a);

    assert_eq!(left.valid_units, right.valid_units);
}

#[test]
fn merge_multiple_stats_accumulates_correctly() {
    let mut stats = ValidationStats::default();

    for _ in 0..100 {
        let s = ValidationStats {
            valid_units: 1,
            invalid_fixed_units: 1,
            invalid_unknown_units: 1,
            missing_units: 1,
            correct_loinc: 1,
            incorrect_loinc: 1,
            unknown_loinc: 1,
            missing_loinc: 1,
        };
        stats.merge(&s);
    }

    assert_eq!(stats.valid_units, 100);
    assert_eq!(stats.invalid_fixed_units, 100);
    assert_eq!(stats.invalid_unknown_units, 100);
    assert_eq!(stats.missing_units, 100);
    assert_eq!(stats.correct_loinc, 100);
    assert_eq!(stats.incorrect_loinc, 100);
    assert_eq!(stats.unknown_loinc, 100);
    assert_eq!(stats.missing_loinc, 100);
}

#[test]
fn print_report_does_not_panic() {
    let stats = ValidationStats {
        valid_units: 5,
        invalid_fixed_units: 1,
        invalid_unknown_units: 2,
        missing_units: 3,
        correct_loinc: 4,
        incorrect_loinc: 5,
        unknown_loinc: 6,
        missing_loinc: 7,
    };

    stats.print_report();
}
