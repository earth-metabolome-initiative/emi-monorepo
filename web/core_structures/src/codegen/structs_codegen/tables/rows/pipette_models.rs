impl From<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::pipette_models::PipetteModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
    ) -> Self {
        super::Rows::PipetteModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
