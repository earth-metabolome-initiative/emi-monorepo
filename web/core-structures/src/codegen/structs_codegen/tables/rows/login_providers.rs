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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::LoginProvider(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
