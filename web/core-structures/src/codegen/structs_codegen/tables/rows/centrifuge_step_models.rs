impl From<crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel,
        >,
    ) -> Self {
        super::Rows::CentrifugeStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugeStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
