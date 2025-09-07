//! Submodule providing traits relative to procedure templates, procedures, and
//! associated builders.

use crate::{database::Read, prelude::Insertable};

/// Trait defining a procedure template.
pub trait ProcedureTemplate<C> {
    /// Associated procedure type.
    type Procedure: Procedure<C, Template = Self>;
    /// Procedure template asset model type.
    type ProcedureTemplateAssetModel: Read<C>;

    /// Returns all procedure template asset models associated with the current
    /// procedure template.
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error>;
}

/// Trait defining a procedure.
pub trait Procedure<C> {
    /// Associated procedure template type.
    type Template: ProcedureTemplate<C, Procedure = Self>;
}

/// Trait defining a procedure builder.
pub trait ProcedureInitializer<C>: ProcedureTemplate<C> {
    /// Root procedure.
    type Root: Procedure<C>;

    /// Returns an initialized builder for the procedure.
    ///
    /// # Arguments
    ///
    /// * `root` - The root procedure from which to determine shared assets and
    ///   asset models, if any. If this is a root procedure being initialized,
    ///   this value should be left to `None`.
    /// * `conn` - A mutable reference to a database connection.
    fn procedure_builder(
        &self,
        root: Option<&Self::Root>,
        conn: &mut C,
    ) -> Result<Self::Procedure, diesel::result::Error>;
}
