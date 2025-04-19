#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableLoginProviderBuilder
{
    type Row = crate::codegen::structs_codegen::tables::login_providers::LoginProvider;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableLoginProvider;
}
