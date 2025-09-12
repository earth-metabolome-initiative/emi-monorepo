impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::soils::Soil
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSoilBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSoil;
}
