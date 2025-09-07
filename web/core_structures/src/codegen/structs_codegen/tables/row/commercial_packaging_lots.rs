impl From<crate::CommercialPackagingLot> for super::Row {
    fn from(value: crate::CommercialPackagingLot) -> Self {
        super::Row::CommercialPackagingLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPackagingLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPackagingLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
