impl From<crate::ProcedureTemplateAssetModel> for super::Rows {
    fn from(value: crate::ProcedureTemplateAssetModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ProcedureTemplateAssetModel>> for super::Rows {
    fn from(value: Vec<crate::ProcedureTemplateAssetModel>) -> Self {
        super::Rows::ProcedureTemplateAssetModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ProcedureTemplateAssetModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureTemplateAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
