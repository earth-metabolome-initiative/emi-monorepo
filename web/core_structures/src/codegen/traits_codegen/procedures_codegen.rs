mod aliquoting_procedures;
mod ball_mill_procedures;
mod capping_procedures;
mod centrifuge_procedures;
mod disposal_procedures;
mod fractioning_procedures;
mod freeze_drying_procedures;
mod freezing_procedures;
mod geolocation_procedures;
mod harvesting_procedures;
mod packaging_procedures;
mod photograph_procedures;
mod placing_procedures;
mod pouring_procedures;
mod procedures;
mod storage_procedures;
mod supernatant_procedures;
mod weighing_procedures;
impl web_common_traits::prelude::ProcedureLike
    for crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureDAG
{
    type Template =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG;
    type ProcedureAsset = crate::ProcedureAsset;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    type Builder =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureBuilderDAG;
    fn procedure_template_asset_models_and_procedure_assets(
        &self,
    ) -> Vec<
        (
            <Self::ProcedureTemplateAssetModel as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
            <Self::ProcedureAsset as web_common_traits::database::PrimaryKeyLike>::PrimaryKey,
        ),
    >{
        match self {
            Self::AliquotingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::BallMillProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::CappingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::CentrifugeProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::DisposalProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::FractioningProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::FreezeDryingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::FreezingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::GeolocationProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::HarvestingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::PackagingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::PhotographProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::PlacingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::PouringProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::Procedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::StorageProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::SupernatantProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
            Self::WeighingProcedure(procedure) => {
                procedure.procedure_template_asset_models_and_procedure_assets()
            }
        }
    }
}
impl web_common_traits::prelude::ProcedureBuilderLike
    for crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureBuilderDAG
{
    type Procedure = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureDAG;
    fn complete_with<G, PT>(
        self,
        parents: &[&PT],
        template: &crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG,
        template_graph: &G,
    ) -> Result<Self, Self::Error>
    where
        G: web_common_traits::prelude::ProcedureTemplateAssetGraph<
                ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel,
                ProcedureAsset = crate::ProcedureAsset,
                ProcedureTemplateRoot = PT,
            >,
    {
        Ok(
            match (self, template) {
                (
                    Self::AliquotingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::AliquotingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::BallMillProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::BallMillProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::CappingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::CappingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::CentrifugeProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::CentrifugeProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::DisposalProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::DisposalProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::FractioningProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::FractioningProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::FreezeDryingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::FreezeDryingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::FreezingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::FreezingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::GeolocationProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::GeolocationProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::HarvestingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::HarvestingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::PackagingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::PackagingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::PhotographProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::PhotographProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::PlacingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::PlacingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::PouringProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::PouringProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::Procedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::ProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::StorageProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::StorageProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::SupernatantProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::SupernatantProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                (
                    Self::WeighingProcedure(builder),
                    crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG::WeighingProcedureTemplate(
                        template,
                    ),
                ) => builder.complete_with(parents, template, template_graph)?.into(),
                _ => {
                    unreachable!(
                        "Mismatched procedure builder and template types, which implies error in code generation."
                    )
                }
            },
        )
    }
}
