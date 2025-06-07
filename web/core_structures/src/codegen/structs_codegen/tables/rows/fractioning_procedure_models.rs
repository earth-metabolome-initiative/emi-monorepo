impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
        >,
    ) -> Self {
        super::Rows::FractioningProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FractioningProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
