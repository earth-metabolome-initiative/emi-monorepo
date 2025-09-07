impl From<crate::Centrifuge> for super::Rows {
    fn from(value: crate::Centrifuge) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Centrifuge>> for super::Rows {
    fn from(value: Vec<crate::Centrifuge>) -> Self {
        super::Rows::Centrifuge(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Centrifuge> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Centrifuge(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
