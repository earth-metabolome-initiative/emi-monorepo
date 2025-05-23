impl From<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ) -> Self {
        super::Row::PackagingModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::packaging_models::PackagingModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
