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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
