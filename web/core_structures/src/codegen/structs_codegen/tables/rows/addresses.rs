impl From<crate::Address> for super::Rows {
    fn from(value: crate::Address) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::Address>> for super::Rows {
    fn from(value: Vec<crate::Address>) -> Self {
        super::Rows::Address(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::Address> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Address(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
