impl From<crate::Asset> for super::Row {
    fn from(value: crate::Asset) -> Self {
        super::Row::Asset(value)
    }
}
impl TryFrom<super::Row> for crate::Asset {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Asset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
