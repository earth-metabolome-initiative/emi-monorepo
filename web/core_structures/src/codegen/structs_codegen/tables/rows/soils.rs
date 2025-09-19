impl From<crate::codegen::structs_codegen::tables::soils::Soil> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::soils::Soil) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::soils::Soil>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::soils::Soil>) -> Self {
        super::Rows::Soil(value)
    }
}
impl From<super::Rows> for Option<Vec<crate::codegen::structs_codegen::tables::soils::Soil>> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::Soil(v) => Some(v),
            _ => None,
        }
    }
}
