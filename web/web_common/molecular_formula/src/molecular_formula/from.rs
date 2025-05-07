//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements::Element;

use super::MolecularFormula;
use crate::{Ion, Solvation};

impl From<Element> for MolecularFormula {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(element)
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

impl From<Solvation> for MolecularFormula {
    fn from(solvation: Solvation) -> Self {
        MolecularFormula::Solvation(Box::new(solvation))
    }
}

impl From<Vec<MolecularFormula>> for MolecularFormula {
    fn from(formulas: Vec<MolecularFormula>) -> Self {
        MolecularFormula::Sequence(formulas)
    }
}

impl From<Vec<MolecularFormula>> for Box<MolecularFormula> {
    fn from(formulas: Vec<MolecularFormula>) -> Self {
        MolecularFormula::Sequence(formulas).into()
    }
}
