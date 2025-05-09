//! Submodule for testing the parsing of molecular formulas.

use std::str::FromStr;

use elements::{Element, Isotope};
use molecular_formula::{Ion, MolecularFormula};

fn test_parse<M: Into<MolecularFormula>>(formula: &str, expected: M, simmetric: Option<&str>) {
    let expected = expected.into();
    let simmetric = simmetric.unwrap_or(formula);
    let parsed_formula = MolecularFormula::from_str(formula).unwrap();
    assert_eq!(parsed_formula, expected, "Failed to parse formula: {formula}");
    assert_eq!(simmetric, &format!("{}", parsed_formula), "Failed to serialize formula: {formula}");
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
        Some("H₂O"),
    );
}

#[test]
/// Test that the `NaCl` formula is parsed correctly
fn test_nacl() {
    test_parse(
        "NaCl",
        MolecularFormula::Sequence(vec![Element::Na.into(), Element::Cl.into()]),
        None,
    );
}

#[test]
fn test_mixture1() {
    test_parse(
        "C+4.H2",
        MolecularFormula::Mixture(vec![
            MolecularFormula::Ion(Ion::from_element(Element::C, 4).unwrap().into()),
            MolecularFormula::Count(Element::H.into(), 2),
        ]),
        Some("C⁺⁴.H₂"),
    );
}

#[test]
fn test_mixture2() {
    test_parse(
        "CH+2.CH+2",
        MolecularFormula::Mixture(vec![
            MolecularFormula::Ion(
                Ion::from_formula(
                    MolecularFormula::Sequence(vec![Element::C.into(), Element::H.into()]),
                    2,
                )
                .unwrap()
                .into(),
            ),
            MolecularFormula::Ion(
                Ion::from_formula(
                    MolecularFormula::Sequence(vec![Element::C.into(), Element::H.into()]),
                    2,
                )
                .unwrap()
                .into(),
            ),
        ]),
        Some("CH⁺².CH⁺²"),
    );
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
        Some("C₁₂H₂₂O₁₁"),
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
        Some("CH₄⁺³"),
    );
}

#[test]
fn test_ion_h() {
    test_parse(
        "H+",
        MolecularFormula::Ion(Ion::from_element(Element::H, 1).unwrap().into()),
        Some("H⁺"),
    );
    test_parse(
        "H-",
        MolecularFormula::Ion(Ion::from_element(Element::H, -1).unwrap().into()),
        Some("H⁻"),
    );
    test_parse(
        "H2+",
        Ion::from_formula(MolecularFormula::Count(Element::H.into(), 2).into(), 1).unwrap(),
        Some("H₂⁺"),
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
        Some("MgSO₄.H₂O"),
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
        Some("(C₁₇H₂₃NO₃)₂.H₂O.H₂SO₄"),
    );
}

#[test]
fn test_formula_including_isotopes() {
    test_parse(
        "¹²C18¹H18¹⁰⁶Pd2³⁵Cl2",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Count(Isotope::try_from((Element::C, 12)).unwrap().into(), 18), /* Isotope with mass number 18 */
            MolecularFormula::Count(Isotope::try_from((Element::H, 1)).unwrap().into(), 18), /* Isotope with mass number 18 */
            MolecularFormula::Count(Isotope::try_from((Element::Pd, 106)).unwrap().into(), 2), /* Isotope with mass number 106 */
            MolecularFormula::Count(Isotope::try_from((Element::Cl, 35)).unwrap().into(), 2), /* Isotope with mass number 35 */
        ]),
        Some("¹²C₁₈¹H₁₈¹⁰⁶Pd₂³⁵Cl₂"),
    )
}

#[test]
fn test_irregular_ion_position1() {
    test_parse(
        "C+4H4",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Ion(Ion::from_element(Element::C, 4).unwrap().into()),
            MolecularFormula::Count(Element::H.into(), 4),
        ]),
        Some("C⁺⁴H₄"),
    )
}

#[test]
fn test_irregular_ion_position2() {
    test_parse(
        "C²⁺H4",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Ion(Ion::from_element(Element::C, 2).unwrap().into()),
            MolecularFormula::Count(Element::H.into(), 4),
        ]),
        Some("C⁺²H₄"),
    )
}

#[test]
fn test_irregular_ion_position3() {
    test_parse(
        "C²⁻H4",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Ion(Ion::from_element(Element::C, -2).unwrap().into()),
            MolecularFormula::Count(Element::H.into(), 4),
        ]),
        Some("C⁻²H₄"),
    )
}

#[test]
fn test_ion1() {
    test_parse("C²⁻", Ion::from_element(Element::C, -2).unwrap(), Some("C⁻²"))
}

#[test]
fn test_ion2() {
    test_parse("C²⁺", Ion::from_element(Element::C, 2).unwrap(), Some("C⁺²"))
}
