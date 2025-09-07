impl From<crate::Procedure> for super::Rows {
    fn from(value: crate::Procedure) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Procedure>> for super::Rows {
    fn from(value: Vec<crate::Procedure>) -> Self {
        super::Rows::Procedure(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Procedure> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Procedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
