#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableLocation;
}
