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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::EmailProvider(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
