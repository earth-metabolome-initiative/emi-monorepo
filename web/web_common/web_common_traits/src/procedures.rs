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

/// Trait defining a graph of procedure templates.
pub trait ProcedureTemplateBuilderGraph<C> {
    /// Procedure template asset model type.
    type ProcedureTemplateAssetModel;
    /// Procedure asset type.
    type ProcedureAsset;
    /// Procedure type.
    type Procedure;
    /// Error type returned when inserting a procedure builder.
    type InsertError;
    /// Procedure template variant type.
    type ProcedureTemplateVariant: ProcedureInitializer<C, ProcedureBuilder = Self::ProcedureBuilderVariant, Graph = Self>;
    /// Procedure builder variant type.
    type ProcedureBuilderVariant: IsCompleteBuilder
        + InsertableVariant<C, UserId = Self::UserId, Error = Self::InsertError>;
    /// User type of the user authoring the procedure template graph.
    type UserId;

    /// Whether the user has requested to autocomplete procedures when
    /// building the graph.
    fn autocomplete(&self) -> bool;

    /// Returns the ID of the user authoring the procedure template graph.
    fn user_id(&self) -> Self::UserId;

    /// Returns the root equivalent of the provided procedure template asset
    /// model.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   for which to find the root equivalent.
    fn root_equivalent(
        &self,
        procedure_template_asset_model: &Self::ProcedureTemplateAssetModel,
    ) -> Option<&Self::ProcedureTemplateAssetModel>;

    /// Returns the procedure asset associated with the provided procedure
    /// template asset model, if any have already been created.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   for which to find the associated procedure asset.
    fn procedure_asset(
        &self,
        procedure_template_asset_model: &Self::ProcedureTemplateAssetModel,
    ) -> Option<&Self::ProcedureAsset>;

    /// Returns the current parent procedure, if any.
    fn parent_procedure(&self) -> Option<&Self::Procedure>;

    /// Returns the last processed procedure, if any.
    fn last_procedure(&self) -> Option<&Self::Procedure>;

    /// Returns a reference to the next procedure template to be processed, if
    /// any.
    fn next_procedure_template(&self) -> Option<Self::ProcedureTemplateVariant>;

    /// Returns the next procedure builder variant to be processed, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a database connection.
    fn next_builder(
        &self,
        conn: &mut C,
    ) -> Result<Option<Self::ProcedureBuilderVariant>, ProcedureTemplateError<Self::InsertError>>
    {
        let Some(procedure_template) = self.next_procedure_template() else {
            return Ok(None);
        };
        let builder = procedure_template.procedure_builder(self, conn)?;

        if builder.is_complete() && self.autocomplete() {
            let _procedure = builder
                .insert(self.user_id(), conn)
                .map_err(ProcedureTemplateError::InsertError)?;
            return self.next_builder(conn);
        }

        Ok(Some(builder))
    }

    /// Returns the next procedure builder forcing it to be of the requested
    /// variant and raises an error if the procedure template does not match
    /// expectations.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a database connection.
    fn try_next_builder<B>(
        &self,
        conn: &mut C,
    ) -> Result<B, ProcedureTemplateError<Self::InsertError>>
    where
        B: TryFrom<
                Self::ProcedureBuilderVariant,
                Error = ProcedureTemplateError<Self::InsertError>,
            >,
    {
        let Some(variant) = self.next_builder(conn)? else {
            return Err(ProcedureTemplateError::NoMoreProcedureBuilders);
        };
        B::try_from(variant)
    }
}

/// Trait defining a procedure builder.
pub trait ProcedureInitializer<C>: ProcedureTemplate<C> {
    /// The procedure builder type.
    type ProcedureBuilder: IsCompleteBuilder + InsertableVariant<C>;
    /// Associated procedure template graph.
    type Graph: ProcedureTemplateBuilderGraph<C>;

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
