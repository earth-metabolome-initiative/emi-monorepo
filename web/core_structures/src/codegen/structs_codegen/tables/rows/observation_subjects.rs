impl From<crate::ObservationSubject> for super::Rows {
    fn from(value: crate::ObservationSubject) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ObservationSubject>> for super::Rows {
    fn from(value: Vec<crate::ObservationSubject>) -> Self {
        super::Rows::ObservationSubject(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ObservationSubject> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ObservationSubject(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
