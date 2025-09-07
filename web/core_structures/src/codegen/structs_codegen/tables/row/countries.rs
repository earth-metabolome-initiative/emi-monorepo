impl From<crate::Country> for super::Row {
    fn from(value: crate::Country) -> Self {
        super::Row::Country(value)
    }
}
impl TryFrom<super::Row> for crate::Country {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Country(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
