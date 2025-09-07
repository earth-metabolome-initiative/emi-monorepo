impl From<crate::PermanenceCategory> for super::Row {
    fn from(value: crate::PermanenceCategory) -> Self {
        super::Row::PermanenceCategory(value)
    }
}
impl TryFrom<super::Row> for crate::PermanenceCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PermanenceCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
