impl From<crate::LoginProvider> for super::Rows {
    fn from(value: crate::LoginProvider) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::LoginProvider>> for super::Rows {
    fn from(value: Vec<crate::LoginProvider>) -> Self {
        super::Rows::LoginProvider(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::LoginProvider> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::LoginProvider(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
