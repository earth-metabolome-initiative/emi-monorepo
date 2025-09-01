impl From<
    crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel,
    ) -> Self {
        super::Row::SharedProcedureTemplateAssetModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SharedProcedureTemplateAssetModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
