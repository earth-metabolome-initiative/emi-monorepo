impl From<crate::CommercialPipetteModel> for super::Rows {
    fn from(value: crate::CommercialPipetteModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPipetteModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialPipetteModel>) -> Self {
        super::Rows::CommercialPipetteModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPipetteModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPipetteModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
