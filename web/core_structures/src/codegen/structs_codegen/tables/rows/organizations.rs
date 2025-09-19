impl From<crate::codegen::structs_codegen::tables::organizations::Organization> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::organizations::Organization) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organizations::Organization>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ) -> Self {
        super::Rows::Organization(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::organizations::Organization>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Organization(v) => Some(v),
            _ => None,
        }
    }
}
