//! Test submodule to verify that the proper error is raised when a
//! superscript is found in the wrong position.

use std::str::FromStr;

use molecular_formulas::{MolecularFormula, errors::Error};

const INCORRECT_SUPERSCRIPT_POSITION: &[&str] = &["H²", "CH²"];

#[test]
/// Test that the error is raised when a superscript is found in the wrong
/// position.
fn test_incorrect_superscript_position() {
    for formula in INCORRECT_SUPERSCRIPT_POSITION {
        assert_eq!(
            MolecularFormula::from_str(formula).unwrap_err(),
            Error::InvalidSuperscriptPosition
        );
    }
}
