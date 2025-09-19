impl From<
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    ) -> Self {
        super::Row::ProcedureTemplateAssetModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ProcedureTemplateAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
