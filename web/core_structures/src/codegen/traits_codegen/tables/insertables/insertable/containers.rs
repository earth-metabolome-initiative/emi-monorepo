impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::containers::Container
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableContainer;
}
