impl From<crate::Freezer> for super::Rows {
    fn from(value: crate::Freezer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Freezer>> for super::Rows {
    fn from(value: Vec<crate::Freezer>) -> Self {
        super::Rows::Freezer(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Freezer> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Freezer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
