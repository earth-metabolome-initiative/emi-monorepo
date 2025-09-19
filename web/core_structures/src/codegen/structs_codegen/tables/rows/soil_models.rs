impl From<crate::codegen::structs_codegen::tables::soil_models::SoilModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::soil_models::SoilModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::soil_models::SoilModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::soil_models::SoilModel>) -> Self {
        super::Rows::SoilModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::soil_models::SoilModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SoilModel(v) => Some(v),
            _ => None,
        }
    }
}
