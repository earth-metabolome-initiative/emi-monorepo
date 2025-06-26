impl From<crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
        >,
    ) -> Self {
        super::Rows::StorageProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::StorageProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
