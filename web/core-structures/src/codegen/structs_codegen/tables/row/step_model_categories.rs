impl From<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
    ) -> Self {
        super::Row::StepModelCategory(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
