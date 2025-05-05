impl From<crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
        >,
    ) -> Self {
        super::Rows::FreezeDryingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
