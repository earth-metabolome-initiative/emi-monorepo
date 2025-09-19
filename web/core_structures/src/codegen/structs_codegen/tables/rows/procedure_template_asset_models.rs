impl From<
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    ) -> Self {
        super::Rows::ProcedureTemplateAssetModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ProcedureTemplateAssetModel(v) => Some(v),
            _ => None,
        }
    }
}
