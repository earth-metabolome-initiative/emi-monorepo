impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::trackables::Trackable
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackable;
}
