#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableNameplateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::nameplates::Nameplate;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableNameplate;
}
