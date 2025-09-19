//! Submodule providing traits relative to procedure templates, procedures, and
//! associated builders.

use crate::database::{DispatchableInsertVariantMetadata, PrimaryKeyLike};

/// Trait defining a root procedure template.
pub trait ProcedureTemplateRoot {
    /// Associated procedure builder type.
    type ProcedureBuilderDAG;

    /// Returns the procedure builder DAG type associated with the current
    /// procedure template.
    fn procedure_builder_dag(&self) -> Self::ProcedureBuilderDAG;

    /// Returns the type of the most concrete procedure as a string slice.
    fn procedure_type(&self) -> &'static str;
}

/// Trait defining a procedure template.
pub trait ProcedureTemplateLike {
    /// Procedure template asset model type.
    type ProcedureTemplateAssetModel;
    /// Associated procedure type.
    type Procedure: ProcedureLike<Template = Self>;
}

/// Trait defining a procedure template.
pub trait ProcedureTemplateQueries<C>: ProcedureTemplateLike {
    /// Returns all procedure template asset models associated with the current
    /// procedure template.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error of type `diesel::result::Error` if the query fails.
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error>;
}

/// Trait defining a procedure.
pub trait ProcedureLike: PrimaryKeyLike {
    /// Associated procedure template type.
    type Template: ProcedureTemplateLike<
            Procedure = Self,
            ProcedureTemplateAssetModel = Self::ProcedureTemplateAssetModel,
        >;
    /// Associated procedure template asset model type.
    type ProcedureTemplateAssetModel: PrimaryKeyLike;
    /// Associated procedure asset type.
    type ProcedureAsset: PrimaryKeyLike;
    /// Associated builder type.
    type Builder: ProcedureBuilderLike<Procedure = Self>;

    #[allow(clippy::type_complexity)]
    /// Returns the coupled procedure template asset model IDs alongside
    /// with their corresponding procedure asset IDs.
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<(
        <Self::ProcedureTemplateAssetModel as PrimaryKeyLike>::PrimaryKey,
        <Self::ProcedureAsset as PrimaryKeyLike>::PrimaryKey,
    )>;
}

/// Trait defining a procedure template asset graph.
pub trait ProcedureTemplateAssetGraph {
    /// The type of the Procedure Template.
    type ProcedureTemplateRoot: ProcedureTemplateRoot;
    /// The type of the Procedure Template Asset Model.
    type ProcedureTemplateAssetModel: PrimaryKeyLike;
    /// The type of the Procedure Asset.
    type ProcedureAsset: PrimaryKeyLike;

    /// Returns the procedure asset ID corresponding to the given procedure
    /// template asset model ID, if any.
    ///
    /// # Arguments
    ///
    /// * `parents` - A slice of references to the parent procedure templates.
    /// * `ptam_id` - The primary key of the procedure template asset model.
    fn procedure_asset(
        &self,
        parents: &[&Self::ProcedureTemplateRoot],
        ptam_id: <Self::ProcedureTemplateAssetModel as PrimaryKeyLike>::PrimaryKey,
    ) -> Option<<Self::ProcedureAsset as PrimaryKeyLike>::PrimaryKey>;
}

/// Trait defining a procedure builder.
pub trait ProcedureBuilderLike:
    Sized + DispatchableInsertVariantMetadata<Row = <Self as ProcedureBuilderLike>::Procedure>
{
    /// Associated procedure type.
    type Procedure: ProcedureLike;

    /// Completes the procedure builder into a procedure using the provided
    /// procedure template.
    ///
    /// # Arguments
    ///
    /// * `parents` - A slice of references to the parent procedure templates.
    /// * `template` - A reference to the procedure template to use for
    ///   completion.
    /// * `template_graph` - A reference to the procedure template asset graph.
    ///
    /// # Errors
    ///
    /// * Returns an error of type `Self::Error` if the completion fails.
    fn complete_with<G, PT>(
        self,
        parents: &[&PT],
        template: &<Self::Procedure as ProcedureLike>::Template,
        template_graph: &G,
    ) -> Result<Self, Self::Error> where
        G: ProcedureTemplateAssetGraph<
            ProcedureTemplateAssetModel = <Self::Procedure as ProcedureLike>::ProcedureTemplateAssetModel,
            ProcedureAsset = <Self::Procedure as ProcedureLike>::ProcedureAsset,
            ProcedureTemplateRoot=PT
        >;
}
