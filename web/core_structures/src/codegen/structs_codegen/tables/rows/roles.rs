impl From<crate::Role> for super::Rows {
    fn from(value: crate::Role) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Role>> for super::Rows {
    fn from(value: Vec<crate::Role>) -> Self {
        super::Rows::Role(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Role> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Role(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
