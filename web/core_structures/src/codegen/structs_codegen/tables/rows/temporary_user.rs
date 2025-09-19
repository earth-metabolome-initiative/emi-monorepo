impl From<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>,
    ) -> Self {
        super::Rows::TemporaryUser(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TemporaryUser(v) => Some(v),
            _ => None,
        }
    }
}
