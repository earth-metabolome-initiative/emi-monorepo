impl From<crate::CommercialPipetteLot> for super::Rows {
    fn from(value: crate::CommercialPipetteLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPipetteLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialPipetteLot>) -> Self {
        super::Rows::CommercialPipetteLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPipetteLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPipetteLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
