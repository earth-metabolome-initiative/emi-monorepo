impl From<
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::WeighingProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::WeighingProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
