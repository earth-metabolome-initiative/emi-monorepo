impl From<crate::codegen::structs_codegen::tables::cities::City> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::cities::City) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cities::City>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cities::City>) -> Self {
        super::Rows::City(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::cities::City>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::City(v) => Some(v),
            _ => None,
        }
    }
}
