impl From<crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent,
    ) -> Self {
        super::Row::ProcedureModelReagent(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelReagent(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
