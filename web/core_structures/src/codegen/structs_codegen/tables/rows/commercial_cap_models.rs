impl From<crate::CommercialCapModel> for super::Rows {
    fn from(value: crate::CommercialCapModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCapModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialCapModel>) -> Self {
        super::Rows::CommercialCapModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCapModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCapModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
