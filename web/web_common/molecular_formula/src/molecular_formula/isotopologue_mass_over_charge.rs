//! Submodule implementing the `isotopologue_mass_over_charge` methods for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Returns the isotopologue mass over charge for the given molecular
    /// formula. Equivalent to `isotologue_mass_with_charge` divided by the
    /// charge.
    pub fn isotologue_mass_over_charge(&self) -> Result<f64, crate::errors::Error> {
        todo!()
    }
}
