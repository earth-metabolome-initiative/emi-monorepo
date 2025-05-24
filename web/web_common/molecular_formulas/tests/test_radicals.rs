//! Submodule testing the correct parsing of radicals in molecular formulas.

use molecular_formulas::{MolecularFormula, Token};

#[test]
/// Test to validate that the appropriate error is raised for invalid radicals.
fn test_invalid_radicals() {
    assert_eq!(
        MolecularFormula::try_from("·"),
        Err(molecular_formulas::errors::Error::EmptyFormula)
    );

    assert_eq!(
        MolecularFormula::try_from("·+"),
        Err(molecular_formulas::errors::Error::InvalidChargePosition)
    );

    assert_eq!(
        MolecularFormula::try_from("-·"),
        Err(molecular_formulas::errors::Error::InvalidChargePosition)
    );

    assert_eq!(
        MolecularFormula::try_from("H2O··"),
        Err(molecular_formulas::errors::Error::InvalidRepeatedToken(Token::Dot))
    );
}

#[test]
fn test_clorine_radical() {
    let formula = MolecularFormula::try_from("Cl·").unwrap();
    assert_eq!(formula.to_string(), "Cl•");
    let formula = MolecularFormula::try_from("•Cl").unwrap();
    assert_eq!(formula.to_string(), "•Cl");
}
