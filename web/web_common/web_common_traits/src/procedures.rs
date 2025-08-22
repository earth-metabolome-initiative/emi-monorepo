//! Submodule providing traits relative to procedure models, procedures, and
//! associated builders.

use crate::prelude::Insertable;

/// Trait defining a procedure model.
pub trait ProcedureModel {
    /// Associated procedure type.
    type Procedure: Procedure<Model = Self>;
}

/// Trait defining a procedure.
pub trait Procedure: Insertable {
    /// Associated procedure model type.
    type Model: ProcedureModel<Procedure = Self>;
}

/// Trait defining a procedure builder.
pub trait ProcedureInitializer<C>: ProcedureModel {
    /// Root procedure model type.
    type Root: ProcedureModel;
    /// Expected error type.
    type Error;

    /// Returns an initialized builder for the procedure.
    ///
    /// # Arguments
    ///
    /// * `root` - The root procedure model from which to determine shared
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
