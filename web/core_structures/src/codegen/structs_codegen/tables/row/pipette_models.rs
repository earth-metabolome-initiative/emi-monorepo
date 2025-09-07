impl From<crate::PipetteModel> for super::Row {
    fn from(value: crate::PipetteModel) -> Self {
        super::Row::PipetteModel(value)
    }
}
impl TryFrom<super::Row> for crate::PipetteModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
