impl From<crate::codegen::structs_codegen::tables::user_emails::UserEmail> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::user_emails::UserEmail) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>) -> Self {
        super::Rows::UserEmail(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::UserEmail(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
