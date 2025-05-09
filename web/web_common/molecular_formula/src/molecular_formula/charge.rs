use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the isotopologue mass of the molecular formula, excluding the
    /// charge.
    pub fn charge(&self) -> Result<i16, crate::errors::Error> {
        todo!("Implement charge calculation");
    }
}
