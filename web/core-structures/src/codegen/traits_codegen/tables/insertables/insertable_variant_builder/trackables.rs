#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder
{
    type Row = crate::codegen::structs_codegen::tables::trackables::Trackable;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTrackable;
}
