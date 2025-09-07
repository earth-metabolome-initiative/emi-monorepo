impl From<crate::CommercialPipetteTipModel> for super::Row {
    fn from(value: crate::CommercialPipetteTipModel) -> Self {
        super::Row::CommercialPipetteTipModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPipetteTipModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteTipModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
