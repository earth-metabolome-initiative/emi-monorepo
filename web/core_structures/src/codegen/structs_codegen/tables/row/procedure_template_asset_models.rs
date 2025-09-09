impl From<
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    ) -> Self {
        super::Row::ProcedureTemplateAssetModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureTemplateAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
