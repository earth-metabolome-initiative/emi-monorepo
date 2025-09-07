impl From<crate::CommercialWeighingDeviceLot> for super::Row {
    fn from(value: crate::CommercialWeighingDeviceLot) -> Self {
        super::Row::CommercialWeighingDeviceLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialWeighingDeviceLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialWeighingDeviceLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
