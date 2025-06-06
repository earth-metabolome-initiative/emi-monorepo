impl From<crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel,
    ) -> Self {
        super::Row::NextProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::NextProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
