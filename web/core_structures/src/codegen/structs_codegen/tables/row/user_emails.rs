impl From<crate::codegen::structs_codegen::tables::user_emails::UserEmail> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::user_emails::UserEmail) -> Self {
        super::Row::UserEmail(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::user_emails::UserEmail> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::UserEmail(v) => Some(v),
            _ => None,
        }
    }
}
