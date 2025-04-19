#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::trackable_states::TrackableState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableStateBuilder;
}
