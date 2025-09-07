impl From<crate::Document> for super::Row {
    fn from(value: crate::Document) -> Self {
        super::Row::Document(value)
    }
}
impl TryFrom<super::Row> for crate::Document {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Document(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
