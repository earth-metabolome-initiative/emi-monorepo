impl From<crate::InstrumentState> for super::Row {
    fn from(value: crate::InstrumentState) -> Self {
        super::Row::InstrumentState(value)
    }
}
impl TryFrom<super::Row> for crate::InstrumentState {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::InstrumentState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
