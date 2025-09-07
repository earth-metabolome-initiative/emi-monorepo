impl From<crate::CommercialBeadModel> for super::Row {
    fn from(value: crate::CommercialBeadModel) -> Self {
        super::Row::CommercialBeadModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialBeadModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
