impl From<crate::FreezeDryer> for super::Row {
    fn from(value: crate::FreezeDryer) -> Self {
        super::Row::FreezeDryer(value)
    }
}
impl TryFrom<super::Row> for crate::FreezeDryer {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
