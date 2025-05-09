//! Represents each molecular formula that can be parsed.

use elements::Element;

use crate::Ion;

mod contains_residual;
mod display;
mod from;
mod from_str;
mod molar_mass;
mod monoisotopic_mass;
mod try_from;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a molecular formula, which can be an element, an ion, a solvate,
/// or a count of molecules.
pub enum MolecularFormula {
    /// An atom (element)
    Element(Element),
    /// An ion (element or molecule with charge)
    Ion(Ion<Box<MolecularFormula>>),
    /// A mixture of molecules
    Mixture(Vec<MolecularFormula>),
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
    #[must_use]
    pub fn chain(self, other: MolecularFormula) -> Self {
        match self {
            Self::Sequence(mut formulas) => {
                formulas.push(other);
                Self::Sequence(formulas)
            }
            _ => Self::Sequence(vec![self, other]),
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
            Self::Mixture(mut formulas) => {
                let first = formulas.first().unwrap().clone();
                let first = first.add_count_to_first_subformula(count)?;
                formulas[0] = first;
                Ok(Self::Mixture(formulas))
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
