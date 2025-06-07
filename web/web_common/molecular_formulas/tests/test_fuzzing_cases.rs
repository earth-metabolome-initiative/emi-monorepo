//! Submodule for testing corner cases identified during fuzzing.
use std::str::FromStr;

use molecular_formulas::{MolecularFormula, errors::Error};

#[test]
fn test_fuzzing_case1() {
    let formula_str = "805F712";
    assert_eq!(MolecularFormula::from_str(formula_str).unwrap_err(), Error::CountingUncountable);
}

#[test]
fn test_fuzzing_case2() {
    let formula_str = "63F6BR.N";
    assert_eq!(MolecularFormula::from_str(formula_str).unwrap_err(), Error::CountingUncountable);
}
