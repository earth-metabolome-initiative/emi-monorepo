impl From<crate::Container> for super::Rows {
    fn from(value: crate::Container) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Container>> for super::Rows {
    fn from(value: Vec<crate::Container>) -> Self {
        super::Rows::Container(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Container> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Container(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
