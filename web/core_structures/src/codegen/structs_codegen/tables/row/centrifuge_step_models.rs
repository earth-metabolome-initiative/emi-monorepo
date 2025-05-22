impl From<crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel,
    ) -> Self {
        super::Row::CentrifugeStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CentrifugeStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
