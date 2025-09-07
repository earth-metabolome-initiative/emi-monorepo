impl From<crate::CommercialBallMillMachineLot> for super::Row {
    fn from(value: crate::CommercialBallMillMachineLot) -> Self {
        super::Row::CommercialBallMillMachineLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialBallMillMachineLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBallMillMachineLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
