impl From<crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
    ) -> Self {
        super::Row::FractioningStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
