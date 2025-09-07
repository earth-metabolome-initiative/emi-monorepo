impl From<crate::EmailProvider> for super::Row {
    fn from(value: crate::EmailProvider) -> Self {
        super::Row::EmailProvider(value)
    }
}
impl TryFrom<super::Row> for crate::EmailProvider {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::EmailProvider(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
