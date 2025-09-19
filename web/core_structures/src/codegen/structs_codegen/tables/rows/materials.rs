impl From<crate::codegen::structs_codegen::tables::materials::Material> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::materials::Material) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::materials::Material>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::materials::Material>) -> Self {
        super::Rows::Material(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::materials::Material>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Material(v) => Some(v),
            _ => None,
        }
    }
}
