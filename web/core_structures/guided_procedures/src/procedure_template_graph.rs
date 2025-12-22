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

impl AsRef<ProcedureTemplateGraph> for ProcedureTemplateGraph {
    fn as_ref(&self) -> &ProcedureTemplateGraph {
        self
    }
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
    pub fn new<C>(
        procedure_template: &ProcedureTemplate,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
        C: LoadConnection,
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
    /// Returns whether the associated procedure template graph is a simple path
    /// which does not branch.
    #[must_use]
    pub fn is_simple_path(&self) -> bool {
        self.task_graphs.iter().all(
            |tg_opt| {
                if let Some(tg) = tg_opt { tg.is_simple_path() } else { true }
            },
        )
    }

    /// Returns whether the provided procedure template asset model is owned by
    /// the root procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   to check ownership for.
    #[must_use]
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
    #[must_use]
    pub fn root_or_foreign_owned_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> bool {
        self.root_owned_ptam(procedure_template_asset_model_id)
            || self.foreign_owned_ptam(procedure_template_asset_model_id)
    }

    /// Returns an iterator over the procedure template asset models which are
    /// either owned by the root procedure template or by the foreign
    /// procedure templates in the hierarchy rooted at the given procedure
    /// template.
    pub fn root_and_foreign_ptams(&self) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.procedure_template_asset_models().filter(|ptam| self.root_or_foreign_owned_ptam(ptam))
    }

    /// Returns the task graph of the given procedure template, if it exists.
    #[must_use]
    pub fn task_graph_of(&self, procedure_template: &ProcedureTemplate) -> Option<&TaskGraph> {
        let procedure_node_id = self.procedure_node_id(procedure_template_id);
        self.task_graphs[procedure_node_id].as_ref()
    }

    /// Returns the closest procedure template which also uses the given
    /// procedure template asset model, as defined by the ownership graph.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parents of the current procedure template, in order
    ///   from the root to the direct parent.
    /// * `parent` - The direct parent of the current procedure template.
    /// * `current` - The current procedure template.
    /// * `reference_ptam` - The procedure template asset model to find the
    ///   closest procedure template for.
    /// * `allow_self` - Whether to allow the current procedure template to be
    ///   returned if it employs the given procedure template asset model.
    #[must_use]
    pub fn closest_paths_to_procedure_template_using_ptam<'graph>(
        &'graph self,
        parents: &[&'graph ProcedureTemplate],
        parent: &'graph ProcedureTemplate,
        current: &'graph ProcedureTemplate,
        reference_ptam: &ProcedureTemplateAssetModel,
        allow_self: bool,
    ) -> Vec<Vec<&'graph ProcedureTemplate>> {
        let mut paths: Vec<Vec<&'graph ProcedureTemplate>> = if let Some(task_graph) =
            self.task_graph_of(current)
        {
            // If this is a recursion step, we check if the current procedure is associated
            // to a task graph. If it is, we search starting from the leaf task nodes
            // whether they employ or contain subprocedures that employ the given
            // procedure template asset model. If it is not, we continue the search
            // recursively in the parents.

            let mut parents = parents.to_vec();
            parents.push(parent);
            task_graph
                .sink_nodes()
                .flat_map(|sink_node| {
                    self.closest_paths_to_procedure_template_using_ptam(
                        &parents,
                        current,
                        sink_node,
                        reference_ptam,
                        true,
                    )
                })
                .collect()
        } else {
            if allow_self {
                let this_complete_parents = {
                    let mut p = parents.to_vec();
                    p.push(parent);
                    p.push(current);
                    p
                };
                for ptam in self.employed_by(current) {
                    if self.reference_based_on_alias(&this_complete_parents, ptam)
                        == Some(reference_ptam)
                    {
                        return vec![this_complete_parents];
                    }
                }
            }

            // We are in a leaf node, so we search through the predecessor of the current
            // node within the task graph of the parent procedure template.
            let task_graph = self.task_graph_of(parent).expect(
                "Parent procedure template must have a task graph if the current one does not.",
            );

            if task_graph.has_predecessors(current) {
                task_graph
                    .predecessors(current)
                    .flat_map(|predecessor| {
                        self.closest_paths_to_procedure_template_using_ptam(
                            parents,
                            parent,
                            predecessor,
                            reference_ptam,
                            true,
                        )
                    })
                    .collect()
            } else {
                // If the current node does not have predecessors, it means we need to move to
                // the parent's predecessors.
                let (grand_parent, parents) = parents.split_last().unwrap();
                let grand_parent_task_graph = self.task_graph_of(grand_parent).expect(
                    "Grand parent procedure template must have a task graph if the parent's does not.",
                );

                grand_parent_task_graph
                    .predecessors(parent)
                    .flat_map(|parent_predecessor| {
                        self.closest_paths_to_procedure_template_using_ptam(
                            parents,
                            grand_parent,
                            parent_predecessor,
                            reference_ptam,
                            true,
                        )
                    })
                    .collect()
            }
        };
        // Then, we deduplicate the paths, as even if the search starts from distinct
        // sink nodes it is possible that they converge to the same procedure template
        // using the given procedure template asset model.

        paths.sort_unstable();
        paths.dedup();

        paths
    }

    /// Returns the leaf node associated to the provided procedure template
    /// which is the root of the root-most task graph which is contained
    /// within the provided procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to find the leaf node
    ///   for.
    #[must_use]
    pub fn root_leaf_node_of<'graph>(
        &'graph self,
        parents: &[&'graph ProcedureTemplate],
        procedure_template: &'graph ProcedureTemplate,
    ) -> (Vec<&'graph ProcedureTemplate>, &'graph ProcedureTemplate) {
        if let Some(task_graph) = self.task_graph_of(procedure_template_id) {
            let root_node = task_graph.root_node();
            let mut parents = parents.to_vec();
            parents.push(procedure_template_id);
            self.root_leaf_node_of(&parents, root_node)
        } else {
            (parents.to_vec(), procedure_template_id)
        }
    }

    /// Returns the leaf nodes associated to the provided procedure template
    /// which are the sinks of the sink-most task graphs which are contained
    /// within the provided procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to find the leaf nodes
    ///   for.
    #[must_use]
    pub fn sink_leaf_nodes_of<'graph>(
        &'graph self,
        parents: &[&'graph ProcedureTemplate],
        procedure_template: &'graph ProcedureTemplate,
    ) -> Vec<(Vec<&'graph ProcedureTemplate>, &'graph ProcedureTemplate)> {
        if let Some(task_graph) = self.task_graph_of(procedure_template_id) {
            let mut result = Vec::new();
            let mut parents = parents.to_vec();
            parents.push(procedure_template_id);
            for sink_node in task_graph.sink_nodes() {
                result.extend(self.sink_leaf_nodes_of(&parents, sink_node));
            }
            result
        } else {
            vec![(parents.to_vec(), procedure_template_id)]
        }
    }
}
