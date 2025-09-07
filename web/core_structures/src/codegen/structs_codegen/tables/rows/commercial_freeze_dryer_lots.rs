impl From<crate::CommercialFreezeDryerLot> for super::Rows {
    fn from(value: crate::CommercialFreezeDryerLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialFreezeDryerLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialFreezeDryerLot>) -> Self {
        super::Rows::CommercialFreezeDryerLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialFreezeDryerLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialFreezeDryerLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
