impl From<crate::codegen::structs_codegen::tables::containers::Container> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::containers::Container) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::containers::Container>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::containers::Container>) -> Self {
        super::Rows::Container(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::containers::Container>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Container(v) => Some(v),
            _ => None,
        }
    }
}
