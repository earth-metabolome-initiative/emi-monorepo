impl From<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    ) -> Self {
        super::Row::ProcedureAsset(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureAsset(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
