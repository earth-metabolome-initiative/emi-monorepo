impl From<crate::codegen::structs_codegen::tables::cap_models::CapModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::cap_models::CapModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>) -> Self {
        super::Rows::CapModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CapModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
