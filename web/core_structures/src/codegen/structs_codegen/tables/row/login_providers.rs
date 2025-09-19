impl From<crate::codegen::structs_codegen::tables::login_providers::LoginProvider> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ) -> Self {
        super::Row::LoginProvider(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::LoginProvider(v) => Some(v),
            _ => None,
        }
    }
}
