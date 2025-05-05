impl From<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>,
    ) -> Self {
        super::Rows::FractioningStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FractioningStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
