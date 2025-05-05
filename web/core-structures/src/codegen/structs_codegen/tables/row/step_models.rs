impl From<crate::codegen::structs_codegen::tables::step_models::StepModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::step_models::StepModel) -> Self {
        super::Row::StepModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::step_models::StepModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
