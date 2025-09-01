impl From<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>,
    ) -> Self {
        super::Rows::PackagingModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
