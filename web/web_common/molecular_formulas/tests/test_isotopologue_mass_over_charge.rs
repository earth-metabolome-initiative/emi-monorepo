//! Submodule testing the computation of the isotopologue mass over charge

use std::str::FromStr;

use molecular_formulas::MolecularFormula;

#[test]
/// Test to check that the `isotopologue_mass_over_charge` method works as
/// expected
fn test_isotopologue_mass_over_charge() {
    let formulas = [("C6H13O6+", 181.07066455650), ("OH-", -17.00328823171)];

    for (formula, expected_isotopologue_mass_over_charge) in formulas {
        let formula = MolecularFormula::from_str(formula).unwrap();
        let isotopologue_mass_over_charge = formula.isotopologue_mass_over_charge().unwrap();

        assert_eq!(
            isotopologue_mass_over_charge, expected_isotopologue_mass_over_charge,
            "Unexpected isotopologue mass over charge for `{formula}`",
        );
    }
}
