#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
{
    type Row = crate::codegen::structs_codegen::tables::email_providers::EmailProvider;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider;
}
