impl From<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::pipette_models::PipetteModel) -> Self {
        super::Row::PipetteModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::pipette_models::PipetteModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
