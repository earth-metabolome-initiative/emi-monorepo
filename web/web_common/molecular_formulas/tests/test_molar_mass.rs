//! Submodule to test the `molar_mass` method of the `MolecularFormula` struct

use std::str::FromStr;

use molecular_formulas::MolecularFormula;

const MOLAR_MASSES: &[(&str, f64)] = &[
    ("H2O", 18.015),
    ("C6H12O6", 180.156),
    ("NaCl", 58.44),
    ("C12H22O11", 342.297),
    ("C2H6O", 46.069),
];

#[test]
/// Test to check that the `molar_mass` method works as expected
fn test_molar_mass() {
    for (formula, expected_molar_mass) in MOLAR_MASSES {
        let formula = MolecularFormula::from_str(formula).unwrap();
        let molar_mass = formula.molar_mass().unwrap();
        assert!(
            (molar_mass - *expected_molar_mass).abs() < 1e-3,
            "Expected molar mass of {formula} to be {expected_molar_mass}",
        );
    }
}
