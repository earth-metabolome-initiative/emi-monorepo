impl From<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
    ) -> Self {
        super::Row::PackagingStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
