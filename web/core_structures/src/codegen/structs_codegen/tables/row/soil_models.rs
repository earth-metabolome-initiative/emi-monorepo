impl From<crate::codegen::structs_codegen::tables::soil_models::SoilModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::soil_models::SoilModel) -> Self {
        super::Row::SoilModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::soil_models::SoilModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SoilModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
