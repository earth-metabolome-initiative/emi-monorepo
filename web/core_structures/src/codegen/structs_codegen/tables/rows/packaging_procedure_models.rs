impl From<
    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
        >,
    ) -> Self {
        super::Rows::PackagingProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
