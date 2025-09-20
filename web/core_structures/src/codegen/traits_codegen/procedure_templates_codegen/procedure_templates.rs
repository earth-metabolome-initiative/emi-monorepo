impl web_common_traits::prelude::ProcedureTemplateRoot
    for crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate
{
    type ProcedureBuilderDAG =
        crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureBuilderDAG;
    fn procedure_builder_dag(&self) -> Self::ProcedureBuilderDAG {
        use web_common_traits::database::Insertable;
        match self.most_concrete_table.as_str() {
            "aliquoting_procedure_templates" => {
                crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure::new()
                    .into()
            }
            "ball_mill_procedure_templates" => {
                crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure::new()
                    .into()
            }
            "capping_procedure_templates" => {
                crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure::new()
                    .into()
            }
            "centrifuge_procedure_templates" => {
                crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure::new()
                    .into()
            }
            "disposal_procedure_templates" => {
                crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure::new()
                    .into()
            }
            "fractioning_procedure_templates" => {
                crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure::new()
                    .into()
            }
            "freeze_drying_procedure_templates" => {
                crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure::new()
                    .into()
            }
            "freezing_procedure_templates" => {
                crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure::new()
                    .into()
            }
            "geolocation_procedure_templates" => {
                crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure::new()
                    .into()
            }
            "harvesting_procedure_templates" => {
                crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure::new()
                    .into()
            }
            "packaging_procedure_templates" => {
                crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure::new()
                    .into()
            }
            "photograph_procedure_templates" => {
                crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure::new()
                    .into()
            }
            "pouring_procedure_templates" => {
                crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure::new()
                    .into()
            }
            "procedure_templates" => {
                crate::codegen::structs_codegen::tables::procedures::Procedure::new()
                    .into()
            }
            "storage_procedure_templates" => {
                crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure::new()
                    .into()
            }
            "supernatant_procedure_templates" => {
                crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure::new()
                    .into()
            }
            "weighing_procedure_templates" => {
                crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure::new()
                    .into()
            }
            most_concrete_column_ident => {
                unreachable!(
                    "Unknown most concrete variant: {most_concrete_column_ident}"
                )
            }
        }
    }
    fn procedure_type(&self) -> &'static str {
        match self.most_concrete_table.as_str() {
            "aliquoting_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
                >()
            }
            "ball_mill_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
                >()
            }
            "capping_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
                >()
            }
            "centrifuge_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
                >()
            }
            "disposal_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
                >()
            }
            "fractioning_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
                >()
            }
            "freeze_drying_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
                >()
            }
            "freezing_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
                >()
            }
            "geolocation_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
                >()
            }
            "harvesting_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
                >()
            }
            "packaging_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
                >()
            }
            "photograph_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
                >()
            }
            "pouring_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
                >()
            }
            "procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::procedures::Procedure,
                >()
            }
            "storage_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
                >()
            }
            "supernatant_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
                >()
            }
            "weighing_procedure_templates" => {
                std::any::type_name::<
                    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
                >()
            }
            most_concrete_column_ident => {
                unreachable!(
                    "Unknown most concrete variant: {most_concrete_column_ident}"
                )
            }
        }
    }
}
impl web_common_traits::prelude::ProcedureTemplateLike
    for crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate
{
    type Procedure = crate::codegen::structs_codegen::tables::procedures::Procedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
}
impl<C> web_common_traits::prelude::ProcedureTemplateQueries<C>
for crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
    Self: web_common_traits::database::MostConcreteVariant<
        C,
        Variant = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG,
    >,
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
        use web_common_traits::database::MostConcreteVariant;
        self.most_concrete_variant(conn)?.procedure_template_asset_models(conn)
    }
}
