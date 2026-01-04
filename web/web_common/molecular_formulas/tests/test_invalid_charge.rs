//! Submodule testing that the appropriate error is raised when attemptinng to
//! create an invalid Ion.

//! Submodule to test the `monoisotopic_mass` method of the `MolecularFormula`
//! struct

use std::str::FromStr;

use molecular_formulas::{MolecularFormula, errors::Error};

const INVALID_OXIDATIVE_STATE: &[(&str, i16)] = &[
    ("H+99", 99),
    ("H+2", 2),
    ("H-2", -2),
    ("H⁺⁹⁹", 99),
    ("H⁺²", 2),
    ("H⁻²", -2),
    ("H²⁺", 2),
    ("H²⁻", -2),
    ("CH+7", 7),
    ("McLv⁺²", 2),
];

const CHARGE_INCORRECT_POSITION: &[&str] = &["C-⁻", "C+⁻", "C⁻-", "C⁻+", "C⁺+", "C⁺+", "⁻C"];

#[test]
/// Test that invalid oxidative states raise the appropriate error when
/// attempting to create an Ion
fn test_invalid_oxidative_state() {
    for (formula, charge) in INVALID_OXIDATIVE_STATE {
        assert_eq!(
            MolecularFormula::from_str(formula).unwrap_err(),
            Error::InvalidOxidationState(*charge),
            "Formula `{formula}` should raise InvalidOxidationState error",
        );
    }
}

#[test]
/// Test to check that when a charge is in the incorrect position, an error is
/// raised.
fn test_charge_incorrect_position() {
    for formula in CHARGE_INCORRECT_POSITION {
        assert_eq!(
            MolecularFormula::from_str(formula),
            Err(Error::InvalidChargePosition),
            "Formula `{formula}` should raise InvalidChargePosition error",
        );
    }
}
