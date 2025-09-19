impl From<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
    ) -> Self {
        super::Rows::ProcedureAsset(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ProcedureAsset(v) => Some(v),
            _ => None,
        }
    }
}
