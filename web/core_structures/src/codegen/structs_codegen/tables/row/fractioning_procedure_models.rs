impl From<
    crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
    ) -> Self {
        super::Row::FractioningProcedureModel(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FractioningProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
