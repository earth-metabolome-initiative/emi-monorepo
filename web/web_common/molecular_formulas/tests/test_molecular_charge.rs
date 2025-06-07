//! Submodule testing the computation of the overall molecular charge.

use std::str::FromStr;

use molecular_formulas::MolecularFormula;

#[test]
fn test_charge() {
    let formula = MolecularFormula::from_str("C6H12O6").unwrap();
    assert_eq!(formula.charge().unwrap(), 0);

    let formula = MolecularFormula::from_str("[Co(NH3)6]+3(Cl−)3").unwrap();
    assert_eq!(formula.charge().unwrap(), 0);

    let formula = MolecularFormula::from_str("H3O+").unwrap();
    assert_eq!(formula.charge().unwrap(), 1);

    let formula = MolecularFormula::from_str("NO2-").unwrap();
    assert_eq!(formula.charge().unwrap(), -1);

    let formula = MolecularFormula::from_str("Ca²⁺").unwrap();
    assert_eq!(formula.charge().unwrap(), 2);
}
