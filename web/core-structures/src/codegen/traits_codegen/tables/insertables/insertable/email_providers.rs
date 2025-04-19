#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::email_providers::EmailProvider
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableEmailProvider;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder;
}
