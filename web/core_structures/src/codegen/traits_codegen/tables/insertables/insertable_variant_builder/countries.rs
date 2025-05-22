#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCountryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::countries::Country;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableCountry;
}
