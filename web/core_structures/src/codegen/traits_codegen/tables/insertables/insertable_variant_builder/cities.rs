#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder
{
    type Row = crate::codegen::structs_codegen::tables::cities::City;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableCity;
}
