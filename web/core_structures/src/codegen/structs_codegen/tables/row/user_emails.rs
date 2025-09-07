impl From<crate::UserEmail> for super::Row {
    fn from(value: crate::UserEmail) -> Self {
        super::Row::UserEmail(value)
    }
}
impl TryFrom<super::Row> for crate::UserEmail {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::UserEmail(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
