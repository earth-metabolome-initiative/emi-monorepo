impl From<crate::CommercialPipetteTipLot> for super::Rows {
    fn from(value: crate::CommercialPipetteTipLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPipetteTipLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialPipetteTipLot>) -> Self {
        super::Rows::CommercialPipetteTipLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPipetteTipLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPipetteTipLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
