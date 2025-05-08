//! Submodule implementing the `Display` trait for the `MolecularFormula` struct

use std::fmt::Display;

use super::MolecularFormula;

impl Display for MolecularFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{element}"),
            Self::Ion(ion) => write!(f, "{ion}"),
            Self::Solvation(solvation) => write!(f, "{solvation}"),
            Self::Count(formula, count) => {
                match formula.as_ref() {
                    Self::Element(_) => {
                        if *count == 1 {
                            unreachable!("Element should not be counted")
                        } else {
                            write!(f, "{formula}{count}")
                        }
                    }
                    _ => {
                        if *count == 1 {
                            unreachable!("Element should not be counted")
                        } else {
                            write!(f, "{count}[{formula}]")
                        }
                    }
                }
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
        }
    }
}
