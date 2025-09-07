impl From<crate::CommercialCapModel> for super::Row {
    fn from(value: crate::CommercialCapModel) -> Self {
        super::Row::CommercialCapModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialCapModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCapModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
