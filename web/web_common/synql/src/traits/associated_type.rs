//! Submodule providing the [`AssociatedType`] trait, which returns the
//! Diesel/Rust type associated with a database object from the set of required
//! crates, if any.

use crate::ExternalType;

/// Trait providing the associated Diesel/Rust type for a database object.
pub trait AssociatedType {
    /// Returns the Diesel/Rust type associated with the object from the set of
    /// required crates, if any.
    ///
    /// # Arguments
    ///
    /// * `crates` - A slice of `ExternalCrate` instances representing the
    ///   crates to check for the associated type.
    /// * `conn` - A mutable reference to a `PgConnection` to use for any
    ///   necessary database queries.
    ///
    /// # Returns
    fn associated_type<'required_crate>(
        &self,
        crates: &'required_crate [crate::ExternalCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<&'required_crate ExternalType>, diesel::result::Error>;
}
