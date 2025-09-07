impl From<crate::City> for super::Rows {
    fn from(value: crate::City) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::City>> for super::Rows {
    fn from(value: Vec<crate::City>) -> Self {
        super::Rows::City(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::City> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::City(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
