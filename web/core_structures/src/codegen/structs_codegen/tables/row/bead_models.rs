impl From<crate::codegen::structs_codegen::tables::bead_models::BeadModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::bead_models::BeadModel) -> Self {
        super::Row::BeadModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::bead_models::BeadModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
