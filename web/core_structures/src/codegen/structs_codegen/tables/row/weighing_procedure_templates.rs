impl From<
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    ) -> Self {
        super::Row::WeighingProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::WeighingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
