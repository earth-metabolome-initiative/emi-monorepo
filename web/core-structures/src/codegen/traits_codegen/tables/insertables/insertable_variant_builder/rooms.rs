#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableRoomBuilder
{
    type Row = crate::codegen::structs_codegen::tables::rooms::Room;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableRoom;
}
