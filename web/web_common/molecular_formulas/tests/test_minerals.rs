//! Submodule testing whether formulas including mineral variants are parsed
//! correctly.
//!
//! The formulas are primarily characterized by having a greek letter in the
//! formula, which does not have any specific chemical meaning, but simply
//! distinguishes the different variants of the same mineral.

use elements_rs::Element;
use molecular_formulas::{GreekLetter, MolecularFormula, errors::Error};

#[test]
/// Test checking that a formula consisting solely of one or more greek letters
/// with and without a minus sign raises the appropriate error.
fn test_only_greek_letter() {
    const IMPROPER_FORMULAS: &[(&str, GreekLetter)] = &[
        ("\u{03b1}", GreekLetter::Alpha),
        ("β", GreekLetter::Beta),
        ("βδ", GreekLetter::Beta),
        ("H2Oβ", GreekLetter::Beta),
        ("H2Oβδ", GreekLetter::Beta),
        ("βH2O", GreekLetter::Beta),
        ("βH2Oδ", GreekLetter::Beta),
        ("H2Oβ-", GreekLetter::Beta),
        ("β-H2O\u{03b1}", GreekLetter::Alpha),
        ("δ-δ", GreekLetter::Delta),
    ];

    for (formula, greek_letter) in IMPROPER_FORMULAS {
        let result = MolecularFormula::try_from(*formula).unwrap_err();
        assert_eq!(
            result,
            Error::InvalidGreekLetterPosition(*greek_letter),
            "Expected error for formula `{formula}` with greek letter `{greek_letter}`"
        );
    }
}

#[test]
fn test_goethite() {
    let formula = "\u{03b1}-FeO(OH)";

    assert_eq!(
        MolecularFormula::try_from(formula)
            .unwrap_or_else(|_| panic!("Failed to parse formula `{formula}`")),
        MolecularFormula::Sequence(vec![
            GreekLetter::Alpha.into(),
            Element::Fe.into(),
            Element::O.into(),
            MolecularFormula::RepeatingUnit(
                MolecularFormula::Sequence(vec![Element::O.into(), Element::H.into()]).into()
            )
        ]),
        "Expected formula `{formula}` to be parsed correctly",
    );
}
