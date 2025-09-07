impl From<crate::CommercialBeadModel> for super::Rows {
    fn from(value: crate::CommercialBeadModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialBeadModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialBeadModel>) -> Self {
        super::Rows::CommercialBeadModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialBeadModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
