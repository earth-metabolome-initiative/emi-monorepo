//! Submodule implementing the `Display` trait for the `MolecularFormula` struct

use std::fmt::Display;

use elements_rs::{ElementVariant, MassNumber};
use fmtastic::{Subscript, Superscript};

use super::{MolecularFormula, Side};

impl Display for MolecularFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(element) => {
                if element.is_lowercase() {
                    write!(f, "{}", element.as_ref().to_string().to_lowercase())
                } else {
                    write!(f, "{}", element.as_ref())
                }
            }
            Self::Isotope(isotope) => {
                let atomic_mass = Superscript(isotope.as_ref().mass_number());
                let element = if isotope.is_lowercase() {
                    isotope.as_ref().element().to_string().to_lowercase()
                } else {
                    isotope.as_ref().element().to_string()
                };
                write!(f, "{atomic_mass}{element}")
            }
            Self::Ion(ion) => write!(f, "{ion}"),
            Self::Mixture(mixture) => {
                write!(
                    f,
                    "{}",
                    mixture.iter().map(MolecularFormula::to_string).collect::<Vec<_>>().join(".")
                )
            }
            Self::Count(formula, count) => {
                let count = Subscript(*count);
                write!(f, "{formula}{count}")
            }
            Self::Sequence(formulas) => {
                assert!(!formulas.is_empty(), "Empty sequence");
                for formula in formulas {
                    write!(f, "{formula}")?;
                }
                Ok(())
            }
            Self::RepeatingUnit(formula) => {
                write!(f, "({formula})")
            }
            Self::Complex(formula) => {
                write!(f, "[{formula}]")
            }
            Self::Residual => {
                write!(f, "R")
            }
            Self::Greek(greek) => {
                write!(f, "{greek}-")
            }
            Self::Radical(formula, side) => {
                match side {
                    Side::Left => write!(f, "•{formula}"),
                    Side::Right => write!(f, "{formula}•"),
                }
            }
        }
    }
}
