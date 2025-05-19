impl From<crate::codegen::structs_codegen::tables::temporary_user_emails::TemporaryUserEmail>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::temporary_user_emails::TemporaryUserEmail,
    ) -> Self {
        super::Row::TemporaryUserEmail(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::temporary_user_emails::TemporaryUserEmail
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TemporaryUserEmail(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
