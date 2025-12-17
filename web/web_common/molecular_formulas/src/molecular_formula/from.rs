//! Submodule implementing several `From` traits for the `MolecularFormula`
//! struct

use elements_rs::{Element, Isotope};

use super::MolecularFormula;
use crate::{
    Ion,
    token::{Atom, greek_letters::GreekLetter},
};

impl From<Element> for MolecularFormula {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(Atom::new(element, false, None))
    }
}

impl From<Element> for Box<MolecularFormula> {
    fn from(element: Element) -> Self {
        MolecularFormula::Element(Atom::new(element, false, None)).into()
    }
}

impl From<Isotope> for MolecularFormula {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(Atom::new(isotope, false, None))
    }
}

impl From<Isotope> for Box<MolecularFormula> {
    fn from(isotope: Isotope) -> Self {
        MolecularFormula::Isotope(Atom::new(isotope, false, None)).into()
    }
}

impl From<GreekLetter> for MolecularFormula {
    fn from(greek_letter: GreekLetter) -> Self {
        MolecularFormula::Greek(greek_letter)
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

impl From<Ion<Atom<Element>>> for MolecularFormula {
    fn from(ion: Ion<Atom<Element>>) -> Self {
        MolecularFormula::Ion(ion.into())
    }
}

impl From<Ion<Atom<Element>>> for Box<MolecularFormula> {
    fn from(ion: Ion<Atom<Element>>) -> Self {
        Box::new(ion.into())
    }
}
