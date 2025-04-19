#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentCategory;
}
