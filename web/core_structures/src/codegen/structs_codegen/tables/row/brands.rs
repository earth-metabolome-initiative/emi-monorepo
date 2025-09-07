impl From<crate::Brand> for super::Row {
    fn from(value: crate::Brand) -> Self {
        super::Row::Brand(value)
    }
}
impl TryFrom<super::Row> for crate::Brand {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Brand(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
