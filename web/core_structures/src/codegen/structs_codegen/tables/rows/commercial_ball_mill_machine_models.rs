impl From<crate::CommercialBallMillMachineModel> for super::Rows {
    fn from(value: crate::CommercialBallMillMachineModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialBallMillMachineModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialBallMillMachineModel>) -> Self {
        super::Rows::CommercialBallMillMachineModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialBallMillMachineModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBallMillMachineModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
