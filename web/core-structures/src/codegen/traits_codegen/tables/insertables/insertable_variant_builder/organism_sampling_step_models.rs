#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismSamplingStepModelBuilder {
    type Row = crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismSamplingStepModel;
}
