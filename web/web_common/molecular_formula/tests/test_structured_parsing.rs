//! Submodule for testing the parsing of molecular formulas.

use std::str::FromStr;

use elements::Element;
use molecular_formula::{Ion, MolecularFormula};

fn test_parse<M: Into<MolecularFormula>>(formula: &str, expected: M) {
    let expected = expected.into();
    let parsed_formula = MolecularFormula::from_str(formula).unwrap();
    assert_eq!(parsed_formula, expected, "Failed to parse formula: {formula}");
    assert_eq!(formula, &format!("{}", parsed_formula), "Failed to serialize formula: {formula}");
}

#[test]
/// Test to check that the `MolecularFormula` struct can be parsed correctly
fn test_h2o() {
    test_parse(
        "H2O",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Count(Element::H.into(), 2),
            Element::O.into(),
        ]),
    );
}

#[test]
/// Test that the `NaCl` formula is parsed correctly
fn test_nacl() {
    test_parse("NaCl", MolecularFormula::Sequence(vec![Element::Na.into(), Element::Cl.into()]));
}

#[test]
/// Test that the `C12H22O11` formula is parsed correctly
fn test_c12h22o11() {
    test_parse(
        "C12H22O11",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Count(Element::C.into(), 12),
            MolecularFormula::Count(Element::H.into(), 22),
            MolecularFormula::Count(Element::O.into(), 11),
        ]),
    );
}

#[test]
/// Test that the `CH4+3` formula is parsed correctly
fn test_ch4_plus_3() {
    test_parse(
        "CH4+3",
        Ion::from_formula(
            MolecularFormula::Sequence(vec![
                Element::C.into(),
                MolecularFormula::Count(Element::H.into(), 4),
            ]),
            3,
        )
        .unwrap(),
    );
}

#[test]
fn test_ion_h() {
    test_parse("H+", MolecularFormula::Ion(Ion::from_element(Element::H, 1).unwrap().into()));
    test_parse("H-", MolecularFormula::Ion(Ion::from_element(Element::H, -1).unwrap().into()));
    test_parse(
        "H2+",
        Ion::from_formula(MolecularFormula::Count(Element::H.into(), 2).into(), 1).unwrap(),
    );
    test_parse(
        "MgSO4.H2O",
        MolecularFormula::Mixture(vec![
            MolecularFormula::Sequence(vec![
                Element::Mg.into(),
                Element::S.into(),
                MolecularFormula::Count(Element::O.into(), 4),
            ]),
            MolecularFormula::Sequence(vec![
                MolecularFormula::Count(Element::H.into(), 2),
                Element::O.into(),
            ]),
        ]),
    );
    test_parse(
        "2(C17H23NO3).H2O.H2SO4",
        MolecularFormula::Mixture(vec![
            MolecularFormula::Count(
                MolecularFormula::RepeatingUnit(
                    MolecularFormula::Sequence(vec![
                        MolecularFormula::Count(Element::C.into(), 17),
                        MolecularFormula::Count(Element::H.into(), 23),
                        Element::N.into(),
                        MolecularFormula::Count(Element::O.into(), 3),
                    ])
                    .into(),
                )
                .into(),
                2,
            ),
            MolecularFormula::Mixture(vec![
                MolecularFormula::Sequence(vec![
                    MolecularFormula::Count(Element::H.into(), 2),
                    Element::O.into(),
                ]),
                MolecularFormula::Sequence(vec![
                    MolecularFormula::Count(Element::H.into(), 2),
                    Element::S.into(),
                    MolecularFormula::Count(Element::O.into(), 4),
                ]),
            ]),
        ]),
    );
}
