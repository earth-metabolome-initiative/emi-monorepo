#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocation;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder;
}
