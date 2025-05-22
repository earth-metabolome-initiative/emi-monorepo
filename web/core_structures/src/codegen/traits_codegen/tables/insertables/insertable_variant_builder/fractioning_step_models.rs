#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepModelBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningStepModel;
}
