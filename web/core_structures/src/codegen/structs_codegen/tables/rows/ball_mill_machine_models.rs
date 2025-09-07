impl From<crate::BallMillMachineModel> for super::Rows {
    fn from(value: crate::BallMillMachineModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::BallMillMachineModel>> for super::Rows {
    fn from(value: Vec<crate::BallMillMachineModel>) -> Self {
        super::Rows::BallMillMachineModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::BallMillMachineModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillMachineModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
