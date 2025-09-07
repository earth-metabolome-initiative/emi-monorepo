impl From<crate::CommercialWeighingDeviceLot> for super::Rows {
    fn from(value: crate::CommercialWeighingDeviceLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialWeighingDeviceLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialWeighingDeviceLot>) -> Self {
        super::Rows::CommercialWeighingDeviceLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialWeighingDeviceLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialWeighingDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
