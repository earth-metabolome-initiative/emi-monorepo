impl From<
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    ) -> Self {
        super::Row::PouringProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PouringProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
