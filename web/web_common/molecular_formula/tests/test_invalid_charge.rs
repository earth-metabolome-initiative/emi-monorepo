//! Submodule testing that the appropriate error is raised when attemptinng to
//! create an invalid Ion.

//! Submodule to test the `monoisotopic_mass` method of the `MolecularFormula`
//! struct

use std::str::FromStr;

use elements::Element;
use molecular_formula::{MolecularFormula, errors::Error};

const INVALID_CHARGES: &[(&str, Element, i8)] =
    &[("H+99", Element::H, 99), ("H+2", Element::H, 2), ("H-2", Element::H, -2)];

#[test]
/// Test to check that the `monoisotopic_mass` method works as expected
fn test_invalid_charge() {
    for (formula, element, charge) in INVALID_CHARGES {
        assert_eq!(
            MolecularFormula::from_str(formula).unwrap_err(),
            Error::InvalidOxidationState(*element, *charge)
        );
    }
}
