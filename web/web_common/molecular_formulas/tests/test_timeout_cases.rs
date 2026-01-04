//! Test submodule to avoid timeout regressions identified via fuzzing.
use std::str::FromStr;

use molecular_formulas::MolecularFormula;

/// This function tests the timeout behavior of the `MolecularFormula::from_str`
/// method.
///
/// # Arguments
///
/// * `formula` - A string slice representing the molecular formula to be
///   parsed.
///
/// # Panics
///
/// * If the parsing of the formula takes longer than 0.5 seconds, it will panic
///   with a message indicating the time taken.
fn timeout_test(formula: &str) {
    let start_time = std::time::Instant::now();
    let _ = MolecularFormula::from_str(formula);
    let elapsed = start_time.elapsed();
    assert!(
        elapsed.as_secs_f64() <= 0.5,
        "Parsing candidate `{formula}` took too long: {} seconds",
        elapsed.as_secs_f64()
    );
}

#[test]
fn test_timeout_case1() {
    timeout_test("6Re427-851");
}

#[test]
fn test_timeout_case2() {
    timeout_test("Cm586P-");
}
