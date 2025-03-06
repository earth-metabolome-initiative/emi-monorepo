#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::nameplates::Nameplate
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplate;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateBuilder;
}
