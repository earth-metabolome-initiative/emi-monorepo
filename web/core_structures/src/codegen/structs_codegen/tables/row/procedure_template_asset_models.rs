impl From<crate::ProcedureTemplateAssetModel> for super::Row {
    fn from(value: crate::ProcedureTemplateAssetModel) -> Self {
        super::Row::ProcedureTemplateAssetModel(value)
    }
}
impl TryFrom<super::Row> for crate::ProcedureTemplateAssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureTemplateAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
