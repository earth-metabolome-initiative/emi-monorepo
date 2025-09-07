impl From<crate::CommercialPipetteTipModel> for super::Rows {
    fn from(value: crate::CommercialPipetteTipModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialPipetteTipModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialPipetteTipModel>) -> Self {
        super::Rows::CommercialPipetteTipModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialPipetteTipModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPipetteTipModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
