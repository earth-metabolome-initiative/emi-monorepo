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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::UserEmail(v) => Some(v),
            _ => None,
        }
    }
}
