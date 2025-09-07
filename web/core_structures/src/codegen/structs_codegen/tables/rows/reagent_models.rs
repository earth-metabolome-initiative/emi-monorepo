impl From<crate::ReagentModel> for super::Rows {
    fn from(value: crate::ReagentModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ReagentModel>> for super::Rows {
    fn from(value: Vec<crate::ReagentModel>) -> Self {
        super::Rows::ReagentModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ReagentModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
