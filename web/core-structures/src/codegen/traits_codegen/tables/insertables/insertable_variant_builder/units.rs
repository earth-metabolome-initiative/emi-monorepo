#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableUnitBuilder
{
    type Row = crate::codegen::structs_codegen::tables::units::Unit;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableUnit;
}
