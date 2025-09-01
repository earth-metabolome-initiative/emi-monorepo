//! Submodule providing traits relative to procedure templates, procedures, and
//! associated builders.

use crate::prelude::Insertable;

/// Trait defining a procedure template.
pub trait ProcedureTemplate {
    /// Associated procedure type.
    type Procedure: Procedure<Model = Self>;
}

/// Trait defining a procedure.
pub trait Procedure: Insertable {
    /// Associated procedure template type.
    type Model: ProcedureTemplate<Procedure = Self>;
}

/// Trait defining a procedure builder.
pub trait ProcedureInitializer<C>: ProcedureTemplate {
    /// Root procedure template type.
    type Root: ProcedureTemplate;
    /// Expected error type.
    type Error;

    /// Returns an initialized builder for the procedure.
    ///
    /// # Arguments
    ///
    /// * `root` - The root procedure template from which to determine shared
    ///   trackables.
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Implementation details
    ///
    /// The builder initialization takes into account any shared trackables
    /// that have already been inserted in previous procedures, ensuring that
    /// the builder is correctly set up to avoid requiring the user to specify
    /// multiple times inferrable fields.
    fn procedure_builder(
        &self,
        root: &Self::Root,
        conn: &mut C,
    ) -> Result<<Self::Procedure as Insertable>::InsertableBuilder, Self::Error>;
}
