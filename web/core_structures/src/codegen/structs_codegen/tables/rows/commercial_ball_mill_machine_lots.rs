impl From<crate::CommercialBallMillMachineLot> for super::Rows {
    fn from(value: crate::CommercialBallMillMachineLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialBallMillMachineLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialBallMillMachineLot>) -> Self {
        super::Rows::CommercialBallMillMachineLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialBallMillMachineLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBallMillMachineLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
