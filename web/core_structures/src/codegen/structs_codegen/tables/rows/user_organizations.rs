impl From<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>,
    ) -> Self {
        super::Rows::UserOrganization(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::UserOrganization(v) => Some(v),
            _ => None,
        }
    }
}
