impl From<crate::LoginProvider> for super::Row {
    fn from(value: crate::LoginProvider) -> Self {
        super::Row::LoginProvider(value)
    }
}
impl TryFrom<super::Row> for crate::LoginProvider {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::LoginProvider(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
