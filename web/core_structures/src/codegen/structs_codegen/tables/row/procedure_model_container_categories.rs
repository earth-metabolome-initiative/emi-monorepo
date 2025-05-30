impl From<
    crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    ) -> Self {
        super::Row::ProcedureModelContainerCategory(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ProcedureModelContainerCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
