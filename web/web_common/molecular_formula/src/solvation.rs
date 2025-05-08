//! This module defines the `Solvation` struct, which represents a solvate
//! molecule.

use std::fmt::Display;

use crate::MolecularFormula;

#[derive(Debug, Clone, PartialEq, Eq)]
/// `Solvation` struct representing a group of molecules composed of a solvate
/// and a solvant. The solvant may appear multiple times in the solvate.
pub struct Solvation {
    /// The solvate molecule that is attached to the solvant.
    pub(crate) solvate: MolecularFormula,
    /// The solvant molecule that is attached to the solvate.
    pub(crate) solvant: MolecularFormula,
}

impl Solvation {
    #[must_use]
    /// Creates a new `Solvation` instance with the given solvate and solvant.
    /// 
    /// # Arguments
    /// 
    /// * `solvate` - A `MolecularFormula` representing the solvate molecule.
    /// * `solvant` - A `MolecularFormula` representing the solvant molecule.
    pub fn new(solvate: MolecularFormula, solvant: MolecularFormula) -> Self {
        Self { solvate, solvant }
    }

    /// Returns whether the solvation contains a residual group.
    pub(crate) fn contains_residual(&self) -> bool {
        self.solvate.contains_residual() || self.solvant.contains_residual()
    }

    pub(crate) fn add_count_to_first_subformula(
        self,
        count: u8,
    ) -> Result<Self, crate::errors::Error> {
        let solvate = self.solvate.add_count_to_first_subformula(count)?;
        Ok(Self { solvate, solvant: self.solvant })
    }
}

impl Display for Solvation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} * {}", self.solvate, self.solvant)
    }
}
