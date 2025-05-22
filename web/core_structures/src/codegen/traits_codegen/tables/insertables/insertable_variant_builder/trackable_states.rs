#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTrackableStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::trackable_states::TrackableState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableState;
}
