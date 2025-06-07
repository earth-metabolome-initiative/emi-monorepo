impl From<crate::codegen::structs_codegen::tables::documents::Document> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::documents::Document) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::documents::Document>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::documents::Document>) -> Self {
        super::Rows::Document(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::documents::Document> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Document(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
