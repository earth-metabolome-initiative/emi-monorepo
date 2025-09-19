impl From<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::freezer_models::FreezerModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>,
    ) -> Self {
        super::Rows::FreezerModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezerModel(v) => Some(v),
            _ => None,
        }
    }
}
