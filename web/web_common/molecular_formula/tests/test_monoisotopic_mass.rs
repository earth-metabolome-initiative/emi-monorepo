//! Submodule to test the `monoisotopic_mass` method of the `MolecularFormula` struct

use std::str::FromStr;

use molecular_formula::MolecularFormula;

const MONOISOTOPIC_MASSES_WITHOUT_CHARGE: &[(&str, f64)] = &[
    ("H2O", 18.01056468403),
    ("OH", 17.0027396518),
    ("OH-", 17.0027396518),
    ("C6H12O6", 180.06338810418),
    ("NaCl", 57.95862196400),
    ("C12H22O11", 342.11621152433),
    ("C2H6O", 46.04186481295),
];

const MONOISOTOPIC_MASSES_WITH_CHARGE: &[(&str, f64)] = &[
    ("H2O", 18.01056468403),
    ("OH", 17.0027396518),
    ("OH-", 17.0032882318),
    ("C6H12O6", 180.06338810418),
    ("NaCl", 57.95862196400),
    ("C12H22O11", 342.11621152433),
    ("C2H6O", 46.04186481295),
];

#[test]
/// Test to check that the `monoisotopic_mass` method works as expected
fn test_isotopologue_mass_without_charge() {
    for (formula, expected_monoisotopic_mass) in MONOISOTOPIC_MASSES_WITHOUT_CHARGE {
        let formula = MolecularFormula::from_str(formula).unwrap();
        let monoisotopic_mass = formula.isotopologue_mass_without_charge().unwrap();

        // We round the exact mass to 11 decimal places to avoid floating point precision issues
        let monoisotopic_mass = (monoisotopic_mass * 1e11).round() / 1e11;

        assert_eq!(
            monoisotopic_mass, *expected_monoisotopic_mass,
            "Expected exact mass of {} to be {}",
            formula, expected_monoisotopic_mass
        );
    }
}

#[test]
/// Test to check that the `monoisotopic_mass` method works as expected
fn test_isotopologue_mass_with_charge() {
    for (formula, expected_monoisotopic_mass) in MONOISOTOPIC_MASSES_WITH_CHARGE {
        let formula = MolecularFormula::from_str(formula).unwrap();
        let monoisotopic_mass = formula.isotopologue_mass_with_charge().unwrap();

        // We round the exact mass to 11 decimal places to avoid floating point precision issues
        let monoisotopic_mass = (monoisotopic_mass * 1e11).round() / 1e11;

        assert_eq!(
            monoisotopic_mass, *expected_monoisotopic_mass,
            "Expected exact mass of {} to be {}",
            formula, expected_monoisotopic_mass
        );
    }
}