impl From<
    crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
        >,
    ) -> Self {
        super::Rows::StepModelTrackableCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelTrackableCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
