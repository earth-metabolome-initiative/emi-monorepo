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
    /// Root procedure.
    type Root: Procedure;
    /// Expected error type.
    type Error;

    /// Returns an initialized builder for the procedure.
    ///
    /// # Arguments
    ///
    /// * `root` - The root procedure from which to determine shared assets and
    ///   asset models.
    /// * `conn` - A mutable reference to a database connection.
    fn procedure_builder(
        &self,
        root: &Self::Root,
        conn: &mut C,
    ) -> Result<<Self::Procedure as Insertable>::InsertableBuilder, Self::Error>;
}
