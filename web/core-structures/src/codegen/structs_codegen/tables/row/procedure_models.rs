impl From<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    ) -> Self {
        super::Row::ProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
