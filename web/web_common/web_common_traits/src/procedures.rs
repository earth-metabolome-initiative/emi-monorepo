//! Submodule providing traits relative to procedure templates, procedures, and
//! associated builders.

use std::rc::Rc;

use common_traits::{builder::IsCompleteBuilder, prelude::Builder};

use crate::{database::Read, prelude::InsertableVariant};

/// Trait defining a procedure template.
pub trait ProcedureTemplate<C> {
    /// Associated procedure type.
    type Procedure: Procedure<C>;
    /// Procedure template asset model type.
    type ProcedureTemplateAssetModel: Read<C>;

    /// Returns all procedure template asset models associated with the current
    /// procedure template.
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error>;
}

impl<C, PT> ProcedureTemplate<C> for &PT
where
    PT: ProcedureTemplate<C>,
{
    type Procedure = PT::Procedure;
    type ProcedureTemplateAssetModel = PT::ProcedureTemplateAssetModel;

    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        (*self).procedure_template_asset_models(conn)
    }
}

impl<C, PT> ProcedureTemplate<C> for Box<PT>
where
    PT: ProcedureTemplate<C>,
{
    type Procedure = PT::Procedure;
    type ProcedureTemplateAssetModel = PT::ProcedureTemplateAssetModel;

    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        (**self).procedure_template_asset_models(conn)
    }
}

impl<C, PT> ProcedureTemplate<C> for Rc<PT>
where
    PT: ProcedureTemplate<C>,
{
    type Procedure = PT::Procedure;
    type ProcedureTemplateAssetModel = PT::ProcedureTemplateAssetModel;

    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        (**self).procedure_template_asset_models(conn)
    }
}

/// Trait defining a procedure.
pub trait Procedure<C> {
    /// Associated procedure template type.
    type Template: ProcedureTemplate<C, Procedure = Self>;
}

/// Enumeration of errors which may happen while working with a procedure
/// template.
pub enum ProcedureTemplateError<InsertErr> {
    /// The procedure builder requested does not match the expected variant.
    ProcedureBuilderMismatch {
        /// The expected builder variant.
        expected: String,
        /// The found builder variant.
        found: String,
    },
    /// An error occurred while inserting a procedure builder.
    InsertError(InsertErr),
    /// No further procedure builders are available.
    NoMoreProcedureBuilders,
    /// Diesel error.
    DieselError(diesel::result::Error),
}

impl<InsertErr> From<diesel::result::Error> for ProcedureTemplateError<InsertErr> {
    fn from(err: diesel::result::Error) -> Self {
        ProcedureTemplateError::DieselError(err)
    }
}

/// Trait defining a builder of graphs of procedure templates.
pub trait ProcedureTemplateBuilderGraphBuilder<'conn, C>: Builder {
    /// Type of the root procedure template
    type ProcedureTemplate;
    /// User type of the user authoring the procedure template graph.
    type UserId;

    /// Whether to autocomplete procedures when building the graph.
    fn autocomplete(self) -> Self;

    /// Root procedure template from which to build the graph.
    fn procedure_template(self, procedure_template: Self::ProcedureTemplate) -> Self;

    /// Sets the user that is autoring the procedute template graph.
    fn user(self, user: Self::UserId) -> Self;

    /// The connection to the database to employ when building the graph.
    fn conn(self, conn: &'conn mut C) -> Self;
}

/// Trait defining a procedure builder.
pub trait ProcedureInitializer<C>: ProcedureTemplate<C> {
    /// The procedure builder type.
    type ProcedureBuilder: IsCompleteBuilder + InsertableVariant<C>;
    /// Associated procedure template graph.
    type Graph;

    /// Returns an initialized builder for the procedure.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph with which to prefil the procedure builder.
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Error
    ///
    /// * If the connection to the database fails.
    fn procedure_builder(
        &self,
        graph: &Self::Graph,
        conn: &mut C,
    ) -> Result<Self::ProcedureBuilder, diesel::result::Error>;
}
