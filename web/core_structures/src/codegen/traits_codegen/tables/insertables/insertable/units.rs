#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::units::Unit
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUnit;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder;
}
