impl From<crate::codegen::structs_codegen::tables::login_providers::LoginProvider> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ) -> Self {
        super::Rows::LoginProvider(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::LoginProvider(v) => Some(v),
            _ => None,
        }
    }
}
