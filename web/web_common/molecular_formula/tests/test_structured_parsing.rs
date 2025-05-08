//! Submodule for testing the parsing of molecular formulas.

use std::str::FromStr;

use elements::Element;
use molecular_formula::MolecularFormula;

fn test_parse(formula: &str, expected: MolecularFormula) {
    let parsed_formula = MolecularFormula::from_str(formula).unwrap();
    assert_eq!(parsed_formula, expected, "Failed to parse formula: {formula}");
}

#[test]
/// Test to check that the `MolecularFormula` struct can be parsed correctly
fn test_h2o() {
    test_parse(
        "H2O",
        vec![MolecularFormula::Count(Element::H.into(), 2), Element::O.into()].into(),
    );
}

#[test]
/// Test that the `NaCl` formula is parsed correctly
fn test_nacl() {
    test_parse("NaCl", vec![Element::Na.into(), Element::Cl.into()].into());
}

#[test]
/// Test that the `C12H22O11` formula is parsed correctly
fn test_c12h22o11() {
    test_parse(
        "C12H22O11",
        vec![
            MolecularFormula::Count(Element::C.into(), 12),
            MolecularFormula::Count(Element::H.into(), 22),
            MolecularFormula::Count(Element::O.into(), 11),
        ]
        .into(),
    );
}
