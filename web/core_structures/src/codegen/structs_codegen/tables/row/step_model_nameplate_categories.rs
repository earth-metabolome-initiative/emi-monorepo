impl From<
    crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    ) -> Self {
        super::Row::StepModelNameplateCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModelNameplateCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
