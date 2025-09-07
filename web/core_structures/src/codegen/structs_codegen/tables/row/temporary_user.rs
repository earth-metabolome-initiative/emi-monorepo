impl From<crate::TemporaryUser> for super::Row {
    fn from(value: crate::TemporaryUser) -> Self {
        super::Row::TemporaryUser(value)
    }
}
impl TryFrom<super::Row> for crate::TemporaryUser {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::TemporaryUser(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
