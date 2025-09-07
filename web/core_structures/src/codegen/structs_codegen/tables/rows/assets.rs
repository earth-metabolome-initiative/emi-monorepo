impl From<crate::Asset> for super::Rows {
    fn from(value: crate::Asset) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Asset>> for super::Rows {
    fn from(value: Vec<crate::Asset>) -> Self {
        super::Rows::Asset(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Asset> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Asset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
