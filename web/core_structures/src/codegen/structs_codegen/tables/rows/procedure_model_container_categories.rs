impl From<
    crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        >,
    ) -> Self {
        super::Rows::ProcedureModelContainerCategory(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ProcedureModelContainerCategory(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
