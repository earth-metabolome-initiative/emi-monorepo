impl From<crate::UserEmail> for super::Rows {
    fn from(value: crate::UserEmail) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::UserEmail>> for super::Rows {
    fn from(value: Vec<crate::UserEmail>) -> Self {
        super::Rows::UserEmail(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::UserEmail> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::UserEmail(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
