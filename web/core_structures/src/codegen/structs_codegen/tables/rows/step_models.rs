impl From<crate::codegen::structs_codegen::tables::step_models::StepModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::step_models::StepModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_models::StepModel>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::step_models::StepModel>) -> Self {
        super::Rows::StepModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::step_models::StepModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
