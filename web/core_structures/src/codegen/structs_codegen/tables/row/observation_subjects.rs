impl From<crate::ObservationSubject> for super::Row {
    fn from(value: crate::ObservationSubject) -> Self {
        super::Row::ObservationSubject(value)
    }
}
impl TryFrom<super::Row> for crate::ObservationSubject {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ObservationSubject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
