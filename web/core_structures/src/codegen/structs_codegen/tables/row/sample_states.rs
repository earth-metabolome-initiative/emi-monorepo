impl From<crate::SampleState> for super::Row {
    fn from(value: crate::SampleState) -> Self {
        super::Row::SampleState(value)
    }
}
impl TryFrom<super::Row> for crate::SampleState {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SampleState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
