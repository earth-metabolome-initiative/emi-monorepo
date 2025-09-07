impl From<crate::Country> for super::Rows {
    fn from(value: crate::Country) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Country>> for super::Rows {
    fn from(value: Vec<crate::Country>) -> Self {
        super::Rows::Country(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Country> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Country(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
