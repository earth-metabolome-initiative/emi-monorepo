impl From<crate::CommercialCentrifugeLot> for super::Rows {
    fn from(value: crate::CommercialCentrifugeLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCentrifugeLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialCentrifugeLot>) -> Self {
        super::Rows::CommercialCentrifugeLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCentrifugeLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCentrifugeLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
