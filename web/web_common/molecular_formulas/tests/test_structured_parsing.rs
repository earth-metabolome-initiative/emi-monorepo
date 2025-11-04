//! Submodule for testing the parsing of molecular formulas.

use std::str::FromStr;

use elements_rs::{Element, Isotope};
use molecular_formulas::{Ion, MolecularFormula};

fn test_parse<M: Into<MolecularFormula>>(formula: &str, expected: M, simmetric: Option<&str>) {
    let expected = expected.into();
    let simmetric = simmetric.unwrap_or(formula);
    let parsed_formula = MolecularFormula::from_str(formula).unwrap_or_else(|err| {
        panic!("Failed to parse formula: `{simmetric}` from `{formula}`, error: {err:?}")
    });
    assert_eq!(
        parsed_formula, expected,
        "Failed to parse formula to the expected output `{formula:#?}`, got `{parsed_formula:#?}`"
    );
    assert_eq!(
        simmetric,
        &format!("{parsed_formula}"),
        "Failed to serialize formula: {formula}, got `{parsed_formula}`"
    );
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
}

#[test]
fn test_molecular_hydrogen() {
    test_parse("H2", MolecularFormula::Count(Element::H.into(), 2), Some("H₂"));
}

#[test]
fn test_hydrogen_molecular_ion() {
    test_parse(
        "H2+",
        Ion::from_formula(MolecularFormula::Count(Element::H.into(), 2), 1).unwrap(),
        Some("H₂⁺"),
    );
}

#[test]
fn test_triatomic_hidrogen() {
    test_parse("H3", MolecularFormula::Count(Element::H.into(), 3), Some("H₃"));
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
    );
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
    );
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
    );
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
    );
}

#[test]
fn test_ion1() {
    test_parse("C²⁻", Ion::from_element(Element::C, -2).unwrap(), Some("C⁻²"));
}

#[test]
fn test_ion2() {
    test_parse("C²⁺", Ion::from_element(Element::C, 2).unwrap(), Some("C⁺²"));
}

#[test]
fn test_methanion() {
    test_parse(
        "CH5+",
        Ion::from_formula(
            MolecularFormula::Sequence(vec![
                Element::C.into(),
                MolecularFormula::Count(Element::H.into(), 5),
            ]),
            1,
        )
        .unwrap(),
        Some("CH₅⁺"),
    );
}

#[test]
fn test_methane_cation() {
    test_parse(
        "CH4+",
        Ion::from_formula(
            MolecularFormula::Sequence(vec![
                Element::C.into(),
                MolecularFormula::Count(Element::H.into(), 4),
            ]),
            1,
        )
        .unwrap(),
        Some("CH₄⁺"),
    );
}

#[test]
fn test_h2so4() {
    test_parse(
        "H2SO4",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Count(Element::H.into(), 2),
            Element::S.into(),
            MolecularFormula::Count(Element::O.into(), 4),
        ]),
        Some("H₂SO₄"),
    );
}

#[test]
fn test_large_compound1() {
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
fn test_atropine() {
    test_parse(
        "C17H23NO3",
        MolecularFormula::Sequence(vec![
            MolecularFormula::Count(Element::C.into(), 17),
            MolecularFormula::Count(Element::H.into(), 23),
            Element::N.into(),
            MolecularFormula::Count(Element::O.into(), 3),
        ]),
        Some("C₁₇H₂₃NO₃"),
    );

    test_parse(
        "(C17H23NO3)",
        MolecularFormula::RepeatingUnit(
            MolecularFormula::Sequence(vec![
                MolecularFormula::Count(Element::C.into(), 17),
                MolecularFormula::Count(Element::H.into(), 23),
                Element::N.into(),
                MolecularFormula::Count(Element::O.into(), 3),
            ])
            .into(),
        ),
        Some("(C₁₇H₂₃NO₃)"),
    );

    test_parse(
        "2(C17H23NO3)",
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
        Some("(C₁₇H₂₃NO₃)₂"),
    );
}

#[test]
fn test_hexaamminecobalt_iii_chloride() {
    test_parse(
        "[Co(NH3)6]+3(Cl−)3",
        MolecularFormula::Sequence(vec![
            Ion::from_formula(
                MolecularFormula::Complex(
                    MolecularFormula::Sequence(vec![
                        Element::Co.into(),
                        MolecularFormula::Count(
                            MolecularFormula::RepeatingUnit(
                                MolecularFormula::Sequence(vec![
                                    Element::N.into(),
                                    MolecularFormula::Count(Element::H.into(), 3),
                                ])
                                .into(),
                            )
                            .into(),
                            6,
                        ),
                    ])
                    .into(),
                ),
                3,
            )
            .unwrap()
            .into(),
            MolecularFormula::Count(
                MolecularFormula::RepeatingUnit(Ion::from_element(Element::Cl, -1).unwrap().into())
                    .into(),
                3,
            ),
        ]),
        Some("[Co(NH₃)₆]⁺³(Cl⁻)₃"),
    );
}
