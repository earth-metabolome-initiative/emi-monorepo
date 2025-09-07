impl From<crate::CentrifugeModel> for super::Rows {
    fn from(value: crate::CentrifugeModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CentrifugeModel>> for super::Rows {
    fn from(value: Vec<crate::CentrifugeModel>) -> Self {
        super::Rows::CentrifugeModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CentrifugeModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CentrifugeModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
