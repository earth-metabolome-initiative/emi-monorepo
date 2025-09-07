impl From<crate::User> for super::Rows {
    fn from(value: crate::User) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::User>> for super::Rows {
    fn from(value: Vec<crate::User>) -> Self {
        super::Rows::User(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::User> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::User(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
