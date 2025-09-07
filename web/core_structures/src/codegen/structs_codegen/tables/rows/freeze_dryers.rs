impl From<crate::FreezeDryer> for super::Rows {
    fn from(value: crate::FreezeDryer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::FreezeDryer>> for super::Rows {
    fn from(value: Vec<crate::FreezeDryer>) -> Self {
        super::Rows::FreezeDryer(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::FreezeDryer> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
