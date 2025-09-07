impl From<crate::TeamState> for super::Row {
    fn from(value: crate::TeamState) -> Self {
        super::Row::TeamState(value)
    }
}
impl TryFrom<super::Row> for crate::TeamState {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TeamState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
