impl From<crate::CommercialPositioningDeviceLot> for super::Row {
    fn from(value: crate::CommercialPositioningDeviceLot) -> Self {
        super::Row::CommercialPositioningDeviceLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPositioningDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPositioningDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
