impl From<crate::Unit> for super::Rows {
    fn from(value: crate::Unit) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Unit>> for super::Rows {
    fn from(value: Vec<crate::Unit>) -> Self {
        super::Rows::Unit(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Unit> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Unit(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
