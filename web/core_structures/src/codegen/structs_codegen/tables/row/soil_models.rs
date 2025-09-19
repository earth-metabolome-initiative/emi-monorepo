impl From<crate::codegen::structs_codegen::tables::soil_models::SoilModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::soil_models::SoilModel) -> Self {
        super::Row::SoilModel(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::soil_models::SoilModel> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SoilModel(v) => Some(v),
            _ => None,
        }
    }
}
