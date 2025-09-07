impl From<crate::Container> for super::Row {
    fn from(value: crate::Container) -> Self {
        super::Row::Container(value)
    }
}
impl TryFrom<super::Row> for crate::Container {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Container(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
