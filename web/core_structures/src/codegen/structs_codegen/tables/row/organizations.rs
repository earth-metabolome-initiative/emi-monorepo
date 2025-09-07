impl From<crate::Organization> for super::Row {
    fn from(value: crate::Organization) -> Self {
        super::Row::Organization(value)
    }
}
impl TryFrom<super::Row> for crate::Organization {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Organization(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
