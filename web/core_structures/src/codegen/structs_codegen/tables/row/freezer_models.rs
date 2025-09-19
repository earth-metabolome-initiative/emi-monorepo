impl From<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freezer_models::FreezerModel) -> Self {
        super::Row::FreezerModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezerModel(v) => Some(v),
            _ => None,
        }
    }
}
