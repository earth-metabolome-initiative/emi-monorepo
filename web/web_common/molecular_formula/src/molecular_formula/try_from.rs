//! Submodule implementing the `TryFrom` trait for the `MolecularFormula` struct

impl TryFrom<&str> for crate::MolecularFormula {
    type Error = crate::errors::Error;

	#[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(s)
    }
}

impl TryFrom<String> for crate::MolecularFormula {
    type Error = crate::errors::Error;

	#[inline]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(&s)
    }
}
