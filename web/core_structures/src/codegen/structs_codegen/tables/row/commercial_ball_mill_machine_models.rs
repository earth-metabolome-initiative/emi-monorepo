impl From<crate::CommercialBallMillMachineModel> for super::Row {
    fn from(value: crate::CommercialBallMillMachineModel) -> Self {
        super::Row::CommercialBallMillMachineModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialBallMillMachineModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBallMillMachineModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
