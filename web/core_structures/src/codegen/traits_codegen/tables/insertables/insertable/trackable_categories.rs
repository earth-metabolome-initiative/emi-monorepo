#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::trackable_categories::TrackableCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableCategoryBuilder;
}
