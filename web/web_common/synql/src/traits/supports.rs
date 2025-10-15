//! Submodule providing the [`Supports`] trait, which indicates whether
//! the Rust type associated with a database object supports the requested
//! trait.

use crate::{AssociatedType, Trait};

/// Trait indicating that the Rust type associated with a database object
/// supports the provided trait.
pub trait Supports {
    /// Returns whether the Rust type associated with the object supports the
    /// trait.
    fn supports(
        &self,
        supported_trait: Trait,
        crates: &[crate::RequiredCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<bool, diesel::result::Error>;
}

impl<T> Supports for T
where
    T: AssociatedType,
{
    fn supports(
        &self,
        supported_trait: Trait,
        crates: &[crate::RequiredCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        Ok(self.associated_type(crates, conn)?.map_or(false, |ty| ty.supports(supported_trait)))
    }
}
