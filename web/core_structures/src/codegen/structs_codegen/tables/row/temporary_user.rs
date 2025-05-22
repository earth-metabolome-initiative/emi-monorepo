impl From<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser) -> Self {
        super::Row::TemporaryUser(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TemporaryUser(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
