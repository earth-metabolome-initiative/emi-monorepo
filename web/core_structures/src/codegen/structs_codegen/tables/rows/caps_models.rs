impl From<crate::codegen::structs_codegen::tables::caps_models::CapsModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::caps_models::CapsModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::caps_models::CapsModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::caps_models::CapsModel>) -> Self {
        super::Rows::CapsModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::caps_models::CapsModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CapsModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
