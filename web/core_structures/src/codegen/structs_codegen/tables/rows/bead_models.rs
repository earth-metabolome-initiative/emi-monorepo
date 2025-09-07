impl From<crate::BeadModel> for super::Rows {
    fn from(value: crate::BeadModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::BeadModel>> for super::Rows {
    fn from(value: Vec<crate::BeadModel>) -> Self {
        super::Rows::BeadModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::BeadModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BeadModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
