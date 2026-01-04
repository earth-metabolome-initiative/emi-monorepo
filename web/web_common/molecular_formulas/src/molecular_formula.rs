//! Represents each molecular formula that can be parsed.

use elements_rs::{Element, Isotope};

use crate::{
    Ion,
    token::{Atom, greek_letters::GreekLetter},
};

mod charge;
mod contains_elements;
mod contains_isotopes;
mod contains_mixtures;
mod contains_residual;
mod countable;
mod diatomic;
mod display;
mod from;
mod from_str;
mod homonuclear;
mod isotopologue_mass;
mod isotopologue_mass_over_charge;
mod molar_mass;
mod noble_gasses;
mod number_of_bonds;
mod oxidation_states;
mod serde;
mod try_from;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents the side in a molecular formula.
pub enum Side {
    /// The left side.
    Left,
    /// The right side.
    Right,
}

#[cfg_attr(feature = "diesel_pgrx", derive(diesel_pgrx::DieselPGRX))]
#[cfg_attr(
    feature = "pgrx",
    derive(pgrx::PostgresType, pgrx::PostgresEq, pgrx::PostgresOrd, pgrx::PostgresHash)
)]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[
    cfg_attr(feature = "diesel", diesel(sql_type = crate::molecular_formula::diesel_impls::MolecularFormula))]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a molecular formula, which can be an element, an ion, a solvate,
/// or a count of molecules.
pub enum MolecularFormula {
    /// An atom (element)
    Element(Atom<Element>),
    /// An isotope (element with mass number)
    Isotope(Atom<Isotope>),
    /// A left-hand side radical.
    Radical(Box<MolecularFormula>, Side),
    /// An ion (element or molecule with charge)
    Ion(Ion<Box<MolecularFormula>>),
    /// A mixture of molecules
    Mixture(Vec<MolecularFormula>),
    /// Number of molecules
    Count(Box<MolecularFormula>, u16),
    /// A sequence of molecular formulas
    Sequence(Vec<MolecularFormula>),
    /// A complex wrapped in square brackets
    Complex(Box<MolecularFormula>),
    /// A repeating unit wrapped in round brackets
    RepeatingUnit(Box<MolecularFormula>),
    /// Residual group
    Residual,
    /// A greek letter decorator
    Greek(GreekLetter),
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

    pub(crate) fn add_count_to_last_subformula(
        self,
        count: u16,
    ) -> Result<Self, crate::errors::Error> {
        match self {
            Self::Sequence(mut formulas) => {
                // TODO! UPDATE THE HANDLE WIKIPEDIA CASE!
                let last = formulas.pop().unwrap();
                let last = last.add_count_to_last_subformula(count)?;
                formulas.push(last);
                Ok(Self::Sequence(formulas))
            }
            Self::Radical(formula, side) => {
                Ok(Self::Radical(Box::new(formula.add_count_to_last_subformula(count)?), side))
            }
            Self::Mixture(mut formulas) => {
                let last = formulas.pop().unwrap();
                let last = last.add_count_to_last_subformula(count)?;
                formulas.push(last);
                Ok(Self::Mixture(formulas))
            }
            Self::Isotope(_)
            | Self::Element(_)
            | Self::Ion(_)
            | Self::Residual
            | Self::Complex(_)
            | Self::RepeatingUnit(_) => Ok(Self::Count(self.into(), count)),
            Self::Count(_, _) | Self::Greek(_) => Err(crate::errors::Error::CountingUncountable),
        }
    }

    pub(crate) fn add_count_to_first_subformula(
        self,
        count: u16,
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
            Self::Isotope(_)
            | Self::Element(_)
            | Self::Ion(_)
            | Self::Complex(_)
            | Self::Radical(_, _)
            | Self::Residual
            | Self::RepeatingUnit(_) => Ok(Self::Count(self.into(), count)),
            Self::Count(_, _) | Self::Greek(_) => Err(crate::errors::Error::CountingUncountable),
        }
    }
}
