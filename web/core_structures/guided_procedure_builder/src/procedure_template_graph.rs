//! Submodule defining the `ProcedureTemplateGraph` structure and its associated
//! methods.

use core_structures::{
    AssetModel, NextProcedureTemplate, ParentProcedureTemplate, ProcedureTemplate,
    ProcedureTemplateAssetModel,
    codegen::diesel_codegen::tables::{
        next_procedure_templates::next_procedure_templates,
        parent_procedure_templates::parent_procedure_templates,
    },
    tables::most_concrete_variants::ProcedureTemplateDAG,
};
use diesel::{
    BelongingToDsl, RunQueryDsl,
    associations::{BelongsTo, HasTable},
    connection::LoadConnection,
    dsl::Asc,
    expression_methods::EqAll,
    query_dsl::{
        LoadQuery,
        methods::{FilterDsl, OrderDsl},
    },
};
use web_common_traits::{database::Read, prelude::MostConcreteVariant};

use crate::structs::{Hierarchy, HierarchyLike, Ownership, OwnershipLike, TaskGraph};

#[derive(Debug, Clone)]
/// Struct providing functionalities to help the user concretely build a
/// procedure.
pub struct ProcedureTemplateGraph {
    /// The task graphs of each procedure template in the hierarchy.
    /// Leaves of the hierarchy have no task graph, hence the `Option`.
    task_graphs: Vec<Option<TaskGraph>>,
    /// The hierarchy of procedure templates, rooted at the procedure template
    /// being built, and including all its sub-procedure templates.
    hierarchy: Hierarchy,
    /// The procedure template asset models associated to the root procedure
    /// template or any of its sub-procedure templates.
    ownership: Ownership,
}

impl ProcedureTemplateGraph {
    /// Creates a new `ProcedureTemplateGraph` rooted at the given procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The root procedure template of the graph.
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an issue querying the
    ///   database.
    pub fn new<C: LoadConnection>(
        procedure_template: &ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
        <NextProcedureTemplate as HasTable>::Table:
            FilterDsl<<next_procedure_templates::parent as EqAll<i32>>::Output>,
        <<NextProcedureTemplate as HasTable>::Table as FilterDsl<
            <next_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output: OrderDsl<(
            Asc<next_procedure_templates::parent>,
            Asc<next_procedure_templates::predecessor>,
            Asc<next_procedure_templates::successor>,
        )>,
        <<<NextProcedureTemplate as HasTable>::Table as FilterDsl<
            <next_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output as OrderDsl<(
            Asc<next_procedure_templates::parent>,
            Asc<next_procedure_templates::predecessor>,
            Asc<next_procedure_templates::successor>,
        )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, NextProcedureTemplate>,
        ProcedureTemplate: web_common_traits::database::Read<C>,
        <ParentProcedureTemplate as HasTable>::Table:
            FilterDsl<<parent_procedure_templates::parent as EqAll<i32>>::Output>,
        <<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output: OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>,
        <<<ParentProcedureTemplate as HasTable>::Table as FilterDsl<
            <parent_procedure_templates::parent as EqAll<i32>>::Output,
        >>::Output as OrderDsl<(
            Asc<parent_procedure_templates::parent>,
            Asc<parent_procedure_templates::child>,
        )>>::Output: RunQueryDsl<C> + for<'a> LoadQuery<'a, C, ParentProcedureTemplate>,
        ProcedureTemplate: web_common_traits::database::Read<C>,
        AssetModel: Read<C>,
        ProcedureTemplateAssetModel: Read<C>,
        C: diesel::connection::LoadConnection,
        ProcedureTemplate: MostConcreteVariant<C, Variant = ProcedureTemplateDAG>,
        ProcedureTemplateAssetModel: BelongsTo<ProcedureTemplate>,
        for<'a> <ProcedureTemplateAssetModel as BelongingToDsl<&'a ProcedureTemplate>>::Output:
            LoadQuery<'a, C, ProcedureTemplateAssetModel>,
    {
        let hierarchy = Hierarchy::new(procedure_template, conn)?;
        let task_graphs = hierarchy.task_graphs(conn)?;
        let ownership = hierarchy.ownership(conn)?;
        Ok(Self { task_graphs, hierarchy, ownership })
    }
}

impl AsRef<Hierarchy> for ProcedureTemplateGraph {
    fn as_ref(&self) -> &Hierarchy {
        &self.hierarchy
    }
}

impl AsRef<Ownership> for ProcedureTemplateGraph {
    fn as_ref(&self) -> &Ownership {
        &self.ownership
    }
}

impl ProcedureTemplateGraph {
    /// Returns whether the provided procedure template asset model is owned by
    /// the root procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   to check ownership for.
    pub fn root_owned_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> bool {
        let root_procedure_template = self.root_procedure_template();
        root_procedure_template.procedure_template
            == procedure_template_asset_model.procedure_template
    }

    /// Returns whether the provided procedure template asset model is owned by
    /// either the root procedure template or by a foreign procedure template in
    /// the hierarchy.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   to check ownership for.
    pub fn root_or_foreign_owned_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> bool {
        self.root_owned_ptam(procedure_template_asset_model)
            || self.foreign_owned_ptam(procedure_template_asset_model)
    }

    /// Returns an iterator over the procedure template asset models which are
    /// either owned by the root procedure template or by the foreign
    /// procedure templates in the hierarchy rooted at the given procedure
    /// template.
    pub fn root_and_foreign_ptams(&self) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.procedure_template_asset_models().filter(|ptam| self.root_or_foreign_owned_ptam(ptam))
    }

    /// Returns the task graph of the given procedure template, if it exists.
    pub fn task_graph_of(&self, procedure_template: &ProcedureTemplate) -> Option<&TaskGraph> {
        let procedure_node_id = self.procedure_node_id(procedure_template);
        self.task_graphs[procedure_node_id].as_ref()
    }

    /// Returns an iterator over the

    /// Returns the root analogue of the given procedure template
    /// asset model.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   to find the analogue for.
    ///
    /// # Panics
    ///
    /// * If the provided procedure template asset model is not owned by either
    ///   the root procedure template
    pub fn root_analogue_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> &ProcedureTemplateAssetModel {
        assert!(self.root_owned_ptam(procedure_template_asset_model));
        todo!()
    }
}
