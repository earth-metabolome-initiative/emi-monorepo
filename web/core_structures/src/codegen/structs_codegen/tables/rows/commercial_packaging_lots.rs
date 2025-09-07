impl From<crate::CommercialPackagingLot> for super::Rows {
    fn from(value: crate::CommercialPackagingLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPackagingLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialPackagingLot>) -> Self {
        super::Rows::CommercialPackagingLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPackagingLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPackagingLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
