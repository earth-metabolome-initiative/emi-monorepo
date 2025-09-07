impl From<crate::PermanenceCategory> for super::Rows {
    fn from(value: crate::PermanenceCategory) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PermanenceCategory>> for super::Rows {
    fn from(value: Vec<crate::PermanenceCategory>) -> Self {
        super::Rows::PermanenceCategory(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PermanenceCategory> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PermanenceCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
