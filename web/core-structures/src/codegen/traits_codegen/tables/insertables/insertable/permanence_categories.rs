#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePermanenceCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePermanenceCategoryBuilder;
}
