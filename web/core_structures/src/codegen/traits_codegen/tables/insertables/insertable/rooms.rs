#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::rooms::Room
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableRoom;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder;
}
