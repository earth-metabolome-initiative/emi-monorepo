impl From<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    ) -> Self {
        super::Row::ProcedureAsset(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ProcedureAsset(v) => Some(v),
            _ => None,
        }
    }
}
