impl From<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::freezer_models::FreezerModel) -> Self {
        super::Row::FreezerModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::freezer_models::FreezerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
