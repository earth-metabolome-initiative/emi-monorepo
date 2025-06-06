impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
    ) -> Self {
        super::Row::PackagingProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
