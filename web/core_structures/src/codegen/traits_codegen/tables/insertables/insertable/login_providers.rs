#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::login_providers::LoginProvider
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder;
}
