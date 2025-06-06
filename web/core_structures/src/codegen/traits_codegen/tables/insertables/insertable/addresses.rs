impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAddress;
}
