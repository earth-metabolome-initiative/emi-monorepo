impl From<
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::PouringProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PouringProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
