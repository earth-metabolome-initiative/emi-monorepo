impl From<crate::Document> for super::Rows {
    fn from(value: crate::Document) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Document>> for super::Rows {
    fn from(value: Vec<crate::Document>) -> Self {
        super::Rows::Document(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Document> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Document(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
