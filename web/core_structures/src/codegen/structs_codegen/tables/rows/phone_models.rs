impl From<crate::PhoneModel> for super::Rows {
    fn from(value: crate::PhoneModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PhoneModel>> for super::Rows {
    fn from(value: Vec<crate::PhoneModel>) -> Self {
        super::Rows::PhoneModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PhoneModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PhoneModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
