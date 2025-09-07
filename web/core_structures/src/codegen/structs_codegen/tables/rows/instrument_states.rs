impl From<crate::InstrumentState> for super::Rows {
    fn from(value: crate::InstrumentState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::InstrumentState>> for super::Rows {
    fn from(value: Vec<crate::InstrumentState>) -> Self {
        super::Rows::InstrumentState(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::InstrumentState> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::InstrumentState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
