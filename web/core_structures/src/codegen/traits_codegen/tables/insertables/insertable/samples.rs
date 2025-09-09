impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::samples::Sample
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSample;
}
