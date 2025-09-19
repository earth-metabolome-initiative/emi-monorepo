impl From<crate::codegen::structs_codegen::tables::users::User> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::users::User) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::users::User>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::users::User>) -> Self {
        super::Rows::User(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::users::User>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::User(v) => Some(v),
            _ => None,
        }
    }
}
