impl From<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
        >,
    ) -> Self {
        super::Rows::PackagingStepModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
