//! Submodule testing that the appropriate error is raised when attemptinng to
//! create an invalid Ion.

//! Submodule to test the `monoisotopic_mass` method of the `MolecularFormula`
//! struct

use std::str::FromStr;

use elements::Element;
use molecular_formula::{MolecularFormula, errors::Error};

const INVALID_OXIDATIVE_STATE: &[(&str, Element, i16)] = &[
    ("H+99", Element::H, 99),
    ("H+2", Element::H, 2),
    ("H-2", Element::H, -2),
    ("H⁺⁹⁹", Element::H, 99),
    ("H⁺²", Element::H, 2),
    ("H⁻²", Element::H, -2),
    ("H²⁺", Element::H, 2),
    ("H²⁻", Element::H, -2),
];

const CHARGE_INCORRECT_POSITION: &[&str] =
    &["C+4H4", "C⁺4H4", "C²⁺H4", "C²⁻H4", "C⁻⁻", "C⁺⁺", "C⁺4", "⁻C"];

#[test]
/// Test that invalid oxidative states raise the appropriate error when
/// attempting to create an Ion
fn test_invalid_oxidative_state() {
    for (formula, element, charge) in INVALID_OXIDATIVE_STATE {
        assert_eq!(
            MolecularFormula::from_str(formula).unwrap_err(),
            Error::InvalidOxidationState(*element, *charge)
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
