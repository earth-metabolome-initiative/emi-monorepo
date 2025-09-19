impl From<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    ) -> Self {
        super::Row::UserOrganization(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::UserOrganization(v) => Some(v),
            _ => None,
        }
    }
}
