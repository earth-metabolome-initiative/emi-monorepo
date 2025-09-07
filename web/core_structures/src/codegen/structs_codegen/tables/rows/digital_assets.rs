impl From<crate::DigitalAsset> for super::Rows {
    fn from(value: crate::DigitalAsset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::DigitalAsset>> for super::Rows {
    fn from(value: Vec<crate::DigitalAsset>) -> Self {
        super::Rows::DigitalAsset(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::DigitalAsset> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::DigitalAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
