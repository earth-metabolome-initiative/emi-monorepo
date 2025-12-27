//! Submodule defining the listener used for the `GuidedProcedureBuilder`.

use std::{collections::HashMap, fmt::Debug};

use common_traits::builder::IsCompleteBuilder;
use core_structures::{
    Procedure, ProcedureAsset, ProcedureTemplate, ProcedureTemplateAssetModel, User,
    tables::{insertables::ProcedureSettable, most_concrete_variants::ProcedureBuilderDAG},
};
use web_common_traits::{
    database::{DispatchableInsertableVariant, MostConcreteVariant, PrimaryKeyLike},
    prelude::{ProcedureLike, ProcedureTemplateAssetGraph, ProcedureTemplateRoot},
    procedures::ProcedureBuilderLike,
};

use crate::{
    PTGListener, ProcedureTemplateGraph,
    guided_procedure::error::{GuidedProcedureError, InternalGuidedProcedureError},
    structs::OwnershipLike,
};
#[derive(Debug)]
pub(super) struct GPBListener<'listener, C> {
    graph: &'listener ProcedureTemplateGraph,
    connection: &'listener mut C,
    author: &'listener User,
    designated_successor: Option<&'listener ProcedureTemplate>,
    parent_procedures: Vec<<Procedure as PrimaryKeyLike>::PrimaryKey>,
    predecessor_procedure: Option<<Procedure as PrimaryKeyLike>::PrimaryKey>,
    procedure_asset_models: HashMap<
        <ProcedureTemplateAssetModel as PrimaryKeyLike>::PrimaryKey,
        <ProcedureAsset as PrimaryKeyLike>::PrimaryKey,
    >,
}

impl<C> ProcedureTemplateAssetGraph for GPBListener<'_, C> {
    type ProcedureTemplateRoot = ProcedureTemplate;
    type ProcedureAsset = ProcedureAsset;
    type ProcedureTemplateAssetModel = ProcedureTemplateAssetModel;

    fn procedure_asset(
        &self,
        parents: &[&Self::ProcedureTemplateRoot],
        ptam_id: <Self::ProcedureTemplateAssetModel as PrimaryKeyLike>::PrimaryKey,
    ) -> Option<<Self::ProcedureAsset as PrimaryKeyLike>::PrimaryKey> {
        let ptam: &ProcedureTemplateAssetModel =
            self.graph.ptam_by_primary_key(ptam_id).expect("PTAM not found in graph");
        let reference_ptam: &ProcedureTemplateAssetModel =
            self.graph.reference_based_on_alias(parents, ptam).expect("Alias not found in graph");
        self.procedure_asset_models.get(&reference_ptam.id).copied()
    }
}

impl<'listener, C> GPBListener<'listener, C> {
    pub(super) fn new(
        graph: &'listener ProcedureTemplateGraph,
        author: &'listener User,
        connection: &'listener mut C,
    ) -> Self {
        Self {
            graph,
            connection,
            author,
            designated_successor: None,
            parent_procedures: Vec::new(),
            predecessor_procedure: None,
            procedure_asset_models: HashMap::new(),
        }
    }

    pub(super) fn connection(&mut self) -> &mut C {
        self.connection
    }

    pub(super) fn insert<B>(
        &mut self,
        parents: &[&'listener ProcedureTemplate],
        builder: B,
    ) -> Result<(), B::Error>
    where
        B: DispatchableInsertableVariant<C> + Debug,
        B::Row: ProcedureLike<
                ProcedureAsset = ProcedureAsset,
                ProcedureTemplateAssetModel = ProcedureTemplateAssetModel,
            > + PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let procedure = builder.insert(self.author.id, self.connection)?;
        self.parent_procedures.push(procedure.primary_key());
        for (ptam_id, pa_id) in
            procedure.procedure_template_asset_models_and_procedure_asset_models()
        {
            let ptam: &ProcedureTemplateAssetModel =
                self.graph.ptam_by_primary_key(ptam_id).expect("PTAM not found in graph");
            let reference_ptam: &ProcedureTemplateAssetModel = self
                .graph
                .reference_based_on_alias(parents, ptam)
                .expect("Alias not found in graph");
            self.procedure_asset_models.insert(reference_ptam.id, pa_id);
        }
        Ok(())
    }
}

impl<'graph, C> PTGListener<'graph> for GPBListener<'graph, C>
where
    ProcedureTemplate: MostConcreteVariant<C>,
    ProcedureBuilderDAG: DispatchableInsertableVariant<C>,
{
    type Output =
        Option<(Vec<&'graph ProcedureTemplate>, &'graph ProcedureTemplate, ProcedureBuilderDAG)>;
    type FilteredSuccessors<I>
        = Option<&'graph core_structures::ProcedureTemplate>
    where
        I: Iterator<Item = &'graph core_structures::ProcedureTemplate>;
    type Error = InternalGuidedProcedureError<'graph>;

    fn enter_foreign_procedure_template(
        &mut self,
        _foreign_procedure_template: &core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        Ok(None)
    }

    fn continue_task(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _predecessors: &[&core_structures::ProcedureTemplate],
        _child: &core_structures::ProcedureTemplate,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enter_procedure_template(
        &mut self,
        parents: &[&'graph core_structures::ProcedureTemplate],
        child: &'graph core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        let most_concrete_child = child.most_concrete_variant(self.connection)?;
        let mut procedure_builder = child
            .procedure_builder_dag()
            .procedure_template(child)?
            .created_by(self.author)?
            .complete_with(parents, &most_concrete_child, self)?;

        if let Some(parent_procedure) = self.parent_procedures.last() {
            procedure_builder = procedure_builder.parent_procedure(parent_procedure)?;
        }

        if let Some(predecessor) = self.predecessor_procedure.as_ref() {
            procedure_builder = procedure_builder.predecessor_procedure(predecessor)?;
        }

        Ok(if procedure_builder.is_complete() {
            self.insert(parents, procedure_builder)?;
            None
        } else {
            Some((parents.to_vec(), child, procedure_builder))
        })
    }

    fn leave_procedure_template(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _child: &core_structures::ProcedureTemplate,
    ) -> Result<Self::Output, Self::Error> {
        self.predecessor_procedure = self.parent_procedures.pop();
        Ok(None)
    }

    fn enter_leaf_ptam(
        &mut self,
        _parents: &[&core_structures::ProcedureTemplate],
        _leaf: &core_structures::ProcedureTemplate,
        _procedure_template_asset_model: &core_structures::ProcedureTemplateAssetModel,
    ) -> Result<Self::Output, Self::Error> {
        Ok(None)
    }

    fn filter_successors<I>(
        &mut self,
        successors: I,
    ) -> Result<Self::FilteredSuccessors<I>, Self::Error>
    where
        I: Iterator<Item = &'graph core_structures::ProcedureTemplate>,
    {
        let successors: Vec<&core_structures::ProcedureTemplate> = successors.collect();

        Ok(match successors.as_slice() {
            [] => None,
            [single_successor] => Some(single_successor),
            _ => {
                let Some(designated_successor) = self.designated_successor else {
                    return Err(InternalGuidedProcedureError::UnclearSuccessor {
                        viable_successors: successors,
                    });
                };
                if successors.contains(&designated_successor) {
                    Some(designated_successor)
                } else {
                    return Err(InternalGuidedProcedureError::DesignatedSuccessorNotFound {
                        designated_successor,
                        viable_successors: successors,
                    });
                }
            }
        })
    }
}
