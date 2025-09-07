impl From<crate::Procedure> for super::Row {
    fn from(value: crate::Procedure) -> Self {
        super::Row::Procedure(value)
    }
}
impl TryFrom<super::Row> for crate::Procedure {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Procedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
