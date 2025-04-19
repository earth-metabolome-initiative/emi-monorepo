#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentCategory;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentCategoryBuilder;
}
