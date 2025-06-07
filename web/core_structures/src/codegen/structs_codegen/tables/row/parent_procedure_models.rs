impl From<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
    ) -> Self {
        super::Row::ParentProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ParentProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
