impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::units::Unit
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUnit;
}
