#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepBuilder
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStep;
}
