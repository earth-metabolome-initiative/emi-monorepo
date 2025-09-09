impl From<crate::codegen::structs_codegen::tables::bead_models::BeadModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::bead_models::BeadModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>) -> Self {
        super::Rows::BeadModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
