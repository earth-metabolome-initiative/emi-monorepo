impl From<crate::CommercialProduct> for super::Rows {
    fn from(value: crate::CommercialProduct) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialProduct>> for super::Rows {
    fn from(value: Vec<crate::CommercialProduct>) -> Self {
        super::Rows::CommercialProduct(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialProduct> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialProduct(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
