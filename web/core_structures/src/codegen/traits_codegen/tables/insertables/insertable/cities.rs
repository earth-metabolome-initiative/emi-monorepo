impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::cities::City
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCity;
}
