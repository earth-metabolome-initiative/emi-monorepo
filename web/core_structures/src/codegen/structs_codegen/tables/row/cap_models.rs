impl From<crate::CapModel> for super::Row {
    fn from(value: crate::CapModel) -> Self {
        super::Row::CapModel(value)
    }
}
impl TryFrom<super::Row> for crate::CapModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CapModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
