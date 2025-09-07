impl From<crate::City> for super::Row {
    fn from(value: crate::City) -> Self {
        super::Row::City(value)
    }
}
impl TryFrom<super::Row> for crate::City {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::City(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
