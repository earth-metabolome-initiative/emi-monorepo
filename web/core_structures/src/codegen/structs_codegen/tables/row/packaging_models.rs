impl From<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ) -> Self {
        super::Row::PackagingModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PackagingModel(v) => Some(v),
            _ => None,
        }
    }
}
