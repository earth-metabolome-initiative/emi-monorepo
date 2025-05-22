#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePermanenceCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertablePermanenceCategory;
}
