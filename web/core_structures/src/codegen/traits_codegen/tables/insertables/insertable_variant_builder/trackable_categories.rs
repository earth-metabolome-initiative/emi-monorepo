#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTrackableCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableCategory;
}
