impl From<crate::SpectraCollection> for super::Row {
    fn from(value: crate::SpectraCollection) -> Self {
        super::Row::SpectraCollection(value)
    }
}
impl TryFrom<super::Row> for crate::SpectraCollection {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SpectraCollection(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
