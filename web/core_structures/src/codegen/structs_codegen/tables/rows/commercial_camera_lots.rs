impl From<crate::CommercialCameraLot> for super::Rows {
    fn from(value: crate::CommercialCameraLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCameraLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialCameraLot>) -> Self {
        super::Rows::CommercialCameraLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCameraLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCameraLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
