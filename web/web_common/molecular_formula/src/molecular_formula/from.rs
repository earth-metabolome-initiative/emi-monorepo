//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements::Element;

use super::MolecularFormula;
use crate::Ion;

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
