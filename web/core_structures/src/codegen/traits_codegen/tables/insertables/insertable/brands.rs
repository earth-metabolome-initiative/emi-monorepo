impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::brands::Brand
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBrand;
}
