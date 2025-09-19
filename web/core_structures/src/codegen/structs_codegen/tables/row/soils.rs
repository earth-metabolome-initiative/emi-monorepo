impl From<crate::codegen::structs_codegen::tables::soils::Soil> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::soils::Soil) -> Self {
        super::Row::Soil(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::soils::Soil> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Soil(v) => Some(v),
            _ => None,
        }
    }
}
