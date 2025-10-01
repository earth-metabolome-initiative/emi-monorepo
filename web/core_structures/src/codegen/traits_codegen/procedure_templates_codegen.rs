mod aliquoting_procedure_templates;
mod ball_mill_procedure_templates;
mod capping_procedure_templates;
mod centrifuge_procedure_templates;
mod disposal_procedure_templates;
mod fractioning_procedure_templates;
mod freeze_drying_procedure_templates;
mod freezing_procedure_templates;
mod geolocation_procedure_templates;
mod harvesting_procedure_templates;
mod packaging_procedure_templates;
mod photograph_procedure_templates;
mod placing_procedure_templates;
mod pouring_procedure_templates;
mod procedure_templates;
mod storage_procedure_templates;
mod supernatant_procedure_templates;
mod weighing_procedure_templates;
impl web_common_traits::prelude::ProcedureTemplateLike
    for crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG
{
    type Procedure = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureDAG;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG
where
    C: diesel::connection::LoadConnection,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<
        crate::ProcedureTemplate,
    >,
    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<
        &'a crate::ProcedureTemplate,
    >>::Output: diesel::query_dsl::LoadQuery<'a, C, crate::ProcedureTemplateAssetModel>,
{
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        Ok(
            match self {
                Self::AliquotingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::BallMillProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::CappingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::CentrifugeProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::DisposalProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::FractioningProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::FreezeDryingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::FreezingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::GeolocationProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::HarvestingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::PackagingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::PhotographProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::PlacingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::PouringProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::StorageProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::SupernatantProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::WeighingProcedureTemplate(value) => {
                    value.procedure_template_asset_models(conn)?
                }
                Self::ProcedureTemplate(value) => {
                    use diesel::{BelongingToDsl, RunQueryDsl};
                    Self::ProcedureTemplateAssetModel::belonging_to(value).load(conn)?
                }
            },
        )
    }
}
