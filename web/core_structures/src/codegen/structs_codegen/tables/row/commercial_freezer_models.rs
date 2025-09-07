impl From<crate::CommercialFreezerModel> for super::Row {
    fn from(value: crate::CommercialFreezerModel) -> Self {
        super::Row::CommercialFreezerModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialFreezerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialFreezerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
