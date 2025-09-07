impl From<crate::PipetteModel> for super::Rows {
    fn from(value: crate::PipetteModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PipetteModel>> for super::Rows {
    fn from(value: Vec<crate::PipetteModel>) -> Self {
        super::Rows::PipetteModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PipetteModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
