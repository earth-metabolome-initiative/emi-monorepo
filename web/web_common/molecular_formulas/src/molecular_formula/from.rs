//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements::{Element, Isotope};

use super::MolecularFormula;
use crate::{Ion, token::greek_letters::GreekLetter};

impl From<Element> for MolecularFormula {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(element)
    }
}

impl From<Isotope> for MolecularFormula {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(isotope)
    }
}

impl From<Isotope> for Box<MolecularFormula> {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(isotope).into()
    }
}

impl From<GreekLetter> for MolecularFormula {
    fn from(greek_letter: GreekLetter) -> Self {
        MolecularFormula::Greek(greek_letter)
    }
}

impl From<Element> for Box<MolecularFormula> {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(element).into()
    }
}

impl From<Ion<Box<MolecularFormula>>> for MolecularFormula {
    fn from(ion: Ion<Box<MolecularFormula>>) -> Self {
        MolecularFormula::Ion(ion)
    }
}

impl From<Ion<MolecularFormula>> for MolecularFormula {
    fn from(ion: Ion<MolecularFormula>) -> Self {
        MolecularFormula::Ion(ion.into())
    }
}

impl From<Ion<Element>> for MolecularFormula {
    fn from(ion: Ion<Element>) -> Self {
        MolecularFormula::Ion(ion.into())
    }
}

impl From<Ion<Element>> for Box<MolecularFormula> {
    fn from(ion: Ion<Element>) -> Self {
        Box::new(ion.into())
    }
}
