impl From<crate::TemporaryUser> for super::Rows {
    fn from(value: crate::TemporaryUser) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::TemporaryUser>> for super::Rows {
    fn from(value: Vec<crate::TemporaryUser>) -> Self {
        super::Rows::TemporaryUser(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::TemporaryUser> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TemporaryUser(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
