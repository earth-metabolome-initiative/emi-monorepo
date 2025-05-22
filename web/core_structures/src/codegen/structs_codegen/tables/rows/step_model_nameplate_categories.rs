impl From<
    crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
        >,
    ) -> Self {
        super::Rows::StepModelNameplateCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModelNameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
