impl From<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep>,
    ) -> Self {
        super::Rows::ShakingStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ShakingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
