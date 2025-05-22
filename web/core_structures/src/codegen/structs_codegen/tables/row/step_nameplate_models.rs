impl From<crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
    ) -> Self {
        super::Row::StepNameplateModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepNameplateModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
