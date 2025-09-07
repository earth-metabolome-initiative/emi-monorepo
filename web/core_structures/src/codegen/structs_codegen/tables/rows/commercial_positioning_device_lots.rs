impl From<crate::CommercialPositioningDeviceLot> for super::Rows {
    fn from(value: crate::CommercialPositioningDeviceLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPositioningDeviceLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialPositioningDeviceLot>) -> Self {
        super::Rows::CommercialPositioningDeviceLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPositioningDeviceLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPositioningDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
