impl From<crate::codegen::structs_codegen::tables::email_providers::EmailProvider> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ) -> Self {
        super::Rows::EmailProvider(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::EmailProvider(v) => Some(v),
            _ => None,
        }
    }
}
