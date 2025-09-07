impl From<crate::ReagentModel> for super::Row {
    fn from(value: crate::ReagentModel) -> Self {
        super::Row::ReagentModel(value)
    }
}
impl TryFrom<super::Row> for crate::ReagentModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
