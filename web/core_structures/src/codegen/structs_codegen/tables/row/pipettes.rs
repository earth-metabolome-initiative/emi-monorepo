impl From<crate::Pipette> for super::Row {
    fn from(value: crate::Pipette) -> Self {
        super::Row::Pipette(value)
    }
}
impl TryFrom<super::Row> for crate::Pipette {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::Pipette(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
