impl From<crate::PipetteTipModel> for super::Rows {
    fn from(value: crate::PipetteTipModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PipetteTipModel>> for super::Rows {
    fn from(value: Vec<crate::PipetteTipModel>) -> Self {
        super::Rows::PipetteTipModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PipetteTipModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PipetteTipModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
