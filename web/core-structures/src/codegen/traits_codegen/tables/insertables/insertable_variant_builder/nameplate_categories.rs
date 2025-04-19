#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableNameplateCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateCategory;
}
