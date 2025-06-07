impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::processables::Processable
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessable;
}
