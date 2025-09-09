impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::countries::Country
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCountry;
}
