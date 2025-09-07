impl From<crate::Color> for super::Rows {
    fn from(value: crate::Color) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Color>> for super::Rows {
    fn from(value: Vec<crate::Color>) -> Self {
        super::Rows::Color(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Color> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Color(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
