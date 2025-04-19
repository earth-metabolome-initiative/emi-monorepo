#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismSamplingStepModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismSamplingStepModelBuilder;
}
