impl From<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser) -> Self {
        super::Row::TemporaryUser(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TemporaryUser(v) => Some(v),
            _ => None,
        }
    }
}
