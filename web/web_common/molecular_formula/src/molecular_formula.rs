//! Represents each molecular formula that can be parsed.

use elements::Element;

use crate::{Ion, Solvation};

mod contains_residual;
mod display;
mod from;
mod from_str;
mod molar_mass;
mod try_from;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a molecular formula, which can be an element, an ion, a solvate,
/// or a count of molecules.
pub enum MolecularFormula {
    /// An atom (element)
    Element(Element),
    /// An ion (element or molecule with charge)
    Ion(Ion<Box<MolecularFormula>>),
    /// A goup of molecules composed of a solvant, a solvate and the count of
    /// solvant
    Solvation(Box<Solvation>),
    /// Number of molecules
    Count(Box<MolecularFormula>, u8),
    /// A sequence of molecular formulas
    Sequence(Vec<MolecularFormula>),
    /// A complex wrapped in square brackets
    Complex(Box<MolecularFormula>),
    /// A repeating unit wrapped in round brackets
    RepeatingUnit(Box<MolecularFormula>),
    /// Residual group
    Residual,
}

impl MolecularFormula {
    /// Chains the provided molecular formula with the current one.
    #[must_use] pub fn chain(self, other: MolecularFormula) -> Self {
        match self {
            Self::Sequence(mut formulas) => {
                formulas.push(other);
                Self::Sequence(formulas)
            }
            _ => Self::Sequence(vec![self, other]),
        }
    }

    /// Returns the last element of the sequence.
    #[must_use] pub fn last_dangling_element(&self) -> Option<&Element> {
        match self {
            Self::Sequence(formulas) => formulas.last().and_then(|f| f.last_dangling_element()),
            Self::Element(element) => Some(element),
            Self::Count(formula, _) => formula.last_dangling_element(),
            Self::Ion(ion) => ion.entry.last_dangling_element(),
            _ => None,
        }
    }

    pub(crate) fn add_count_to_first_subformula(
        self,
        count: u8,
    ) -> Result<Self, crate::errors::Error> {
        match self {
            Self::Sequence(mut formulas) => {
                let first = formulas.first().unwrap().clone();
                let first = first.add_count_to_first_subformula(count)?;
                formulas[0] = first;
                Ok(Self::Sequence(formulas))
            }
            Self::Solvation(solvation) => {
                Ok(Self::Solvation(solvation.add_count_to_first_subformula(count)?.into()))
            }
            Self::Element(_) | Self::Ion(_) | Self::Complex(_) | Self::RepeatingUnit(_) => {
                Ok(Self::Count(self.into(), count))
            }
            Self::Count(_, _) => {
                unreachable!("Count {self:?} should not be counted")
            }
            Self::Residual => Ok(Self::Count(self.into(), count)),
        }
    }
}
