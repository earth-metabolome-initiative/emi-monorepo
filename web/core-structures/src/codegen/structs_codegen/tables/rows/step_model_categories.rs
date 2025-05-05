impl From<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        >,
    ) -> Self {
        super::Rows::StepModelCategory(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
