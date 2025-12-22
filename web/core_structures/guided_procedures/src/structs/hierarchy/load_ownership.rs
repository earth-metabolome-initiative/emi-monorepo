//! Submodule defining a function to recursively load the sub-procedure
//! templates of a given procedure template.

use std::rc::Rc;

use algebra::impls::CSR2D;
use core_structures::{
    AssetModel, ProcedureTemplate, ProcedureTemplateAssetModel,
    tables::most_concrete_variants::ProcedureTemplateDAG,
};
use diesel::{BelongingToDsl, associations::BelongsTo, query_dsl::LoadQuery};
use graph::{
    prelude::{GenericBiGraph, GenericEdgesBuilder, MonopartiteGraph},
    traits::EdgesBuilder,
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::{
    database::Read,
    prelude::{Builder, MostConcreteVariant},
    procedures::ProcedureTemplateQueries,
};

use crate::structs::{Hierarchy, Ownership};

impl Hierarchy {
    /// Returns the ownership bipartite graph of the procedure templates in the
    /// hierarchy with the procedure template asset models they reference.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Errors
    ///
    /// * Returns a `diesel::result::Error` if there is an issue querying the
    ///   database.
    pub(crate) fn ownership<C>(&self, conn: &mut C) -> Result<Ownership, diesel::result::Error>
    where
        ProcedureTemplateAssetModel: Read<C>,
        C: diesel::connection::LoadConnection,
        ProcedureTemplate: MostConcreteVariant<C, Variant = ProcedureTemplateDAG>,
        ProcedureTemplateAssetModel: BelongsTo<ProcedureTemplate>,
        ProcedureTemplate: Read<C>,
        for<'a> <ProcedureTemplateAssetModel as BelongingToDsl<&'a ProcedureTemplate>>::Output:
            LoadQuery<'a, C, ProcedureTemplateAssetModel>,
        AssetModel: Read<C>,
    {
        let mut foreign_procedure_templates = Vec::new();
        let mut procedure_template_asset_models = Vec::new();
        let mut edges = Vec::new();

        for (i, procedure_template_id) in self.hierarchy.nodes_vocabulary().iter().enumerate() {
            for ptam in procedure_template.procedure_template_asset_models(conn)? {
                // If the owner of the procedure template asset model is not in
                // the hierarchy, add it to the foreign procedure templates.
                if self
                    .hierarchy
                    .nodes_vocabulary()
                    .binary_search_by(|pt| {
                        pt.as_ref().procedure_template.cmp(&ptam.procedure_template_id)
                    })
                    .is_err()
                {
                    let ptam_owner = ptam.procedure_template(conn)?;
                    foreign_procedure_templates.push(ptam_owner);
                }

                edges.push((i, ptam.clone()));
                procedure_template_asset_models.push(ptam);
            }
        }

        foreign_procedure_templates.sort_unstable();
        foreign_procedure_templates.dedup();
        let foreign_procedure_templates = SortedVec::try_from(foreign_procedure_templates).unwrap();
        procedure_template_asset_models.sort_unstable();
        procedure_template_asset_models.dedup();
        let procedure_template_asset_models =
            SortedVec::try_from(procedure_template_asset_models).unwrap();

        let mut edges = edges
            .into_iter()
            .map(|(i, ptam)| (i, procedure_template_asset_models.binary_search(&ptam).unwrap()))
            .collect::<Vec<(usize, usize)>>();

        edges.sort_unstable();
        edges.dedup();

        let edges: CSR2D<usize, usize, usize> = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len())
            .expected_shape((
                self.hierarchy.nodes_vocabulary().len(),
                procedure_template_asset_models.len(),
            ))
            .edges(edges)
            .build()
            .expect("Failed to build ownership edges");

        let graph = GenericBiGraph::try_from((
            self.hierarchy.nodes_vocabulary().clone(),
            Rc::new(procedure_template_asset_models),
            edges,
        ))
        .expect("Failed to build ownership graph");

        Ownership::new(graph, foreign_procedure_templates, conn)
    }
}
