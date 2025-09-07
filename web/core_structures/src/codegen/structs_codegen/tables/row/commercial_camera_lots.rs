impl From<crate::CommercialCameraLot> for super::Row {
    fn from(value: crate::CommercialCameraLot) -> Self {
        super::Row::CommercialCameraLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialCameraLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCameraLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
