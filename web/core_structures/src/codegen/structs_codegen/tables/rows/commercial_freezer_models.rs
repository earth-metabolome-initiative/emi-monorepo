impl From<crate::CommercialFreezerModel> for super::Rows {
    fn from(value: crate::CommercialFreezerModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialFreezerModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialFreezerModel>) -> Self {
        super::Rows::CommercialFreezerModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialFreezerModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialFreezerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
