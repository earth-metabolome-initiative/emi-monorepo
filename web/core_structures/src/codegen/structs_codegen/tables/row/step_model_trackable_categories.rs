impl From<
    crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
    ) -> Self {
        super::Row::StepModelTrackableCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelTrackableCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
