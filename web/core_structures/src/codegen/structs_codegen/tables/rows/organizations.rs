impl From<crate::Organization> for super::Rows {
    fn from(value: crate::Organization) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Organization>> for super::Rows {
    fn from(value: Vec<crate::Organization>) -> Self {
        super::Rows::Organization(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Organization> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Organization(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
