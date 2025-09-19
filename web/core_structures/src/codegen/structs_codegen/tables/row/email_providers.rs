impl From<crate::codegen::structs_codegen::tables::email_providers::EmailProvider> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    ) -> Self {
        super::Row::EmailProvider(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::EmailProvider(v) => Some(v),
            _ => None,
        }
    }
}
