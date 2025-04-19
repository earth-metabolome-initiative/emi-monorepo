#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::trackables::Trackable
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackable;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder;
}
