//! Submodule defining a bipartite graph representing procedure templates and
//! the procedure template asset models they reference.

use std::rc::Rc;

use algebra::impls::{CSR2D, GenericBiMatrix2D, SquareCSR2D};
use core_structures::{AssetModel, ProcedureTemplate, ProcedureTemplateAssetModel};
use diesel::connection::LoadConnection;
use graph::{
    prelude::{
        GenericBiGraph, GenericEdgesBuilder, GenericGraph, GenericMonoplexMonopartiteGraphBuilder,
    },
    traits::{
        BipartiteGraph, EdgesBuilder, MonopartiteGraph, MonopartiteGraphBuilder, MonoplexGraph,
        MonoplexGraphBuilder, TransposedMonoplexGraph,
    },
};
use sorted_vec::prelude::SortedVec;
use web_common_traits::{database::Read, prelude::Builder};

#[derive(Debug, Clone)]
#[allow(clippy::type_complexity)]
/// Bipartite graph representing procedure templates and their asset models.
pub struct Ownership {
    /// Bipartite graph.
    graph: GenericBiGraph<
        Rc<SortedVec<Rc<ProcedureTemplate>>>,
        Rc<SortedVec<ProcedureTemplateAssetModel>>,
        CSR2D<usize, usize, usize>,
    >,
    /// Foreign procedure template which were not included in the hierarchy
    /// but are referenced by the procedure templates asset models.
    foreign_procedure_templates: SortedVec<ProcedureTemplate>,
    /// Asset models referenced in the procedure template asset models.
    asset_models: Vec<AssetModel>,
    /// Graph describing the procedure template asset models which are
    /// based on other procedure template asset models.
    derivatives: GenericGraph<
        Rc<SortedVec<ProcedureTemplateAssetModel>>,
        GenericBiMatrix2D<
            SquareCSR2D<CSR2D<usize, usize, usize>>,
            SquareCSR2D<CSR2D<usize, usize, usize>>,
        >,
    >,
}

impl Ownership {
    #[allow(clippy::type_complexity)]
    pub(super) fn new<C: LoadConnection>(
        graph: GenericBiGraph<
            Rc<SortedVec<Rc<ProcedureTemplate>>>,
            Rc<SortedVec<ProcedureTemplateAssetModel>>,
            CSR2D<usize, usize, usize>,
        >,
        foreign_procedure_templates: SortedVec<ProcedureTemplate>,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>
    where
        AssetModel: Read<C>,
    {
        let mut edges = Vec::new();
        let ptams = graph.right_nodes_vocabulary().clone();
        for (i, ptam) in ptams.iter().enumerate() {
            if let Some(original_ptam) = ptam.based_on {
                let original_index = ptams
                    .binary_search_by(|am| am.id.cmp(&original_ptam))
                    .expect("Based on asset model not found in vocabulary");
                edges.push((i, original_index));
            }
        }

        edges.sort_unstable();
        edges.dedup();

        let asset_models = ptams
            .iter()
            .map(|ptam| ptam.asset_model(conn))
            .collect::<Result<Vec<AssetModel>, diesel::result::Error>>()?;

        let number_of_nodes = ptams.len();
        let directed: SquareCSR2D<CSR2D<usize, usize, usize>> = GenericEdgesBuilder::default()
            .expected_number_of_edges(edges.len())
            .expected_shape(number_of_nodes)
            .edges(edges)
            .build()
            .expect("Failed to build ownership graph");
        let derivatives = GenericMonoplexMonopartiteGraphBuilder::default()
            .nodes(ptams)
            .edges(GenericBiMatrix2D::new(directed))
            .build()
            .expect("Failed to build ownership graph");

        Ok(Self { graph, foreign_procedure_templates, asset_models, derivatives })
    }
}

/// Trait providing read-only access to an `Ownership` structure.
pub trait OwnershipLike: AsRef<Ownership> {
    /// Returns the number of procedure template asset models in the ownership
    /// graph.
    fn number_of_procedure_template_asset_models(&self) -> usize {
        self.as_ref().graph.number_of_right_nodes()
    }

    /// Returns the number of foreign procedure templates in the ownership
    /// graph.
    fn number_of_foreign_procedure_templates(&self) -> usize {
        self.as_ref().foreign_procedure_templates.len()
    }

    /// Returns whether the provided procedure template is a foreign procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template to check.
    fn is_foreign_procedure_template(&self, procedure_template: &ProcedureTemplate) -> bool {
        self.as_ref().foreign_procedure_templates.binary_search(procedure_template_id).is_ok()
    }

    /// Returns an iterator over all procedure template asset models in the
    /// ownership graph.
    fn procedure_template_asset_models(
        &self,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        self.as_ref().graph.right_nodes_vocabulary().iter()
    }

    /// Returns an iterator over all foreign procedure templates in the
    /// ownership graph.
    fn foreign_procedure_templates(&self) -> core::slice::Iter<'_, ProcedureTemplate> {
        self.as_ref().foreign_procedure_templates.iter()
    }

    /// Returns the optional reference associated with the foreign procedure
    /// owning the given procedure template asset model, if any.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose foreign owning procedure template is to be retrieved.
    fn foreign_procedure_template_of(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> Option<&ProcedureTemplate> {
        self.as_ref()
            .foreign_procedure_templates
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template_asset_model.procedure_template_id)
            })
            .ok()
            .and_then(|index| self.as_ref().foreign_procedure_templates.get(index))
    }

    /// Returns whether the given procedure template asset model is owned by a
    /// foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose ownership status is to be checked.
    fn foreign_owned_ptam(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> bool {
        self.foreign_procedure_template_of(procedure_template_asset_model_id).is_some()
    }

    /// Returns the procedure template asset models which are owned by the
    /// provided foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The foreign procedure template whose owned
    ///   procedure template asset models are to be retrieved.
    fn foreign_ptams_of(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        assert!(self.is_foreign_procedure_template(procedure_template_id));
        self.procedure_template_asset_models()
            .filter(move |ptam| ptam.procedure_template == procedure_template.procedure_template_id)
    }

    /// Returns the procedure template asset models which are employed by
    /// the provided procedure template.
    ///
    /// # Arguments
    ///
    /// * `procedure_template` - The procedure template leaf whose employed
    ///   procedure template asset models are to be retrieved.
    ///
    /// # Panics
    ///
    /// * If the provided procedure template is not part of the ownership graph.
    fn employed_by(
        &self,
        procedure_template: &ProcedureTemplate,
    ) -> impl Iterator<Item = &ProcedureTemplateAssetModel> {
        let procedure_template_id = self
            .as_ref()
            .graph
            .left_nodes_vocabulary()
            .binary_search_by(|pt| {
                pt.procedure_template.cmp(&procedure_template.procedure_template_id)
            })
            .expect("Procedure template not part of ownership graph");

        self.as_ref().graph.successors(procedure_template_id).map(|ptam_id| {
            self.as_ref()
                .graph
                .right_nodes_vocabulary()
                .get(ptam_id)
                .expect("Procedure template asset model id out of bounds")
        })
    }

    /// Returns a reference to the asset model owned by the given procedure
    /// template asset model.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose owned asset model is to be retrieved.
    ///
    /// # Panics
    ///
    /// * If the provided procedure template asset model is not part of the
    ///   ownership graph.
    fn asset_model_of(
        &self,
        procedure_template_asset_model: &ProcedureTemplateAssetModel,
    ) -> &AssetModel {
        let ptam_id = self
            .as_ref()
            .graph
            .right_nodes_vocabulary()
            .binary_search(procedure_template_asset_model_id)
            .expect("Procedure template asset model not part of ownership graph");

        self.as_ref().asset_models.get(ptam_id).expect("Asset model id out of bounds")
    }

    /// Returns the ptam associated with the provided identifier.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The identifier of the procedure template asset model
    ///   to retrieve.
    fn ptam_by_primary_key(
        &self,
        primary_key: <ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
    ) -> Option<&ProcedureTemplateAssetModel> {
        self.as_ref()
            .graph
            .right_nodes_vocabulary()
            .binary_search_by(|ptam: &ProcedureTemplateAssetModel| ptam.id.cmp(&primary_key))
            .ok()
            .and_then(|index| self.as_ref().graph.right_nodes_vocabulary().get(index))
    }

    /// Returns the certain based on aliases of the given procedure
    /// template asset model.
    ///
    /// # Arguments
    /// * `parents` - The parent procedure templates of the given procedure
    ///   template asset model.
    /// * `procedure_template_asset_model` - The procedure template asset model
    ///   whose certain based on aliases are to be retrieved.
    ///
    /// # Implementation details
    ///
    /// Given a procedure template asset model A, if it has only a single
    /// ancestor B which is based on A, then B is considered an certain
    /// based on alias of A. If A has multiple ancestors, then none of them
    /// are considered certain based on aliases, as it is unclear which
    /// one is the true alias.
    fn reference_based_on_alias<'graph>(
        &'graph self,
        parents: &[&ProcedureTemplate],
        procedure_template_asset_model: &'graph ProcedureTemplateAssetModel,
    ) -> Option<&'graph ProcedureTemplateAssetModel> {
        if procedure_template_asset_model.procedure_template == parents[0].procedure_template {
            // If the PTAM is owned by the root procedure template, it is its own
            // certain based on alias.
            return Some(procedure_template_asset_model_id);
        }

        let nv = self.as_ref().derivatives.nodes_vocabulary();

        let ptam_id = nv
            .binary_search(procedure_template_asset_model_id)
            .expect("Procedure template asset model not part of ownership graph");

        // Either the current PTAM has a single predecessor, or if they are
        // multiple ones we employ the parents to disambiguate which one
        // is the true alias.

        let certain_based_on_alias = if self.as_ref().derivatives.in_degree(ptam_id) == 1 {
            nv.get(self.as_ref().derivatives.predecessors(ptam_id).next()?)
                .expect("Procedure template asset model id out of bounds")
        } else {
            let mut certain_based_on_alias = None;
            for predecessor in self.as_ref().derivatives.predecessors(ptam_id) {
                let predecessor_pt =
                    nv.get(predecessor).expect("Procedure template asset model id out of bounds");

                for parent in parents {
                    if predecessor_pt.procedure_template == parent.procedure_template {
                        certain_based_on_alias = Some(predecessor_pt);
                        break;
                    }
                }
            }
            certain_based_on_alias?
        };

        self.reference_based_on_alias(parents, certain_based_on_alias)
    }
}

impl<T: AsRef<Ownership>> OwnershipLike for T {}

impl AsRef<Ownership> for Ownership {
    fn as_ref(&self) -> &Ownership {
        self
    }
}
