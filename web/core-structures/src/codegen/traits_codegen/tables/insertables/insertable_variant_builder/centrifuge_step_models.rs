#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepModel;
}
