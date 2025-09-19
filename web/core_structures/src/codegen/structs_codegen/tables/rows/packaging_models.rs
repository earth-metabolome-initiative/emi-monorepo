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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PackagingModel(v) => Some(v),
            _ => None,
        }
    }
}
