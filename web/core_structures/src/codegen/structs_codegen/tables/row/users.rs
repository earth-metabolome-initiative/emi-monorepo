impl From<crate::codegen::structs_codegen::tables::users::User> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::users::User) -> Self {
        super::Row::User(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::users::User> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::User(v) => Some(v),
            _ => None,
        }
    }
}
