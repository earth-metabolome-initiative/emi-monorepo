impl From<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    ) -> Self {
        super::Row::StorageProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StorageProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
