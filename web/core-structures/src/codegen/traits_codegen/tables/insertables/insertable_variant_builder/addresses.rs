#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    type Row = crate::codegen::structs_codegen::tables::addresses::Address;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableAddress;
}
