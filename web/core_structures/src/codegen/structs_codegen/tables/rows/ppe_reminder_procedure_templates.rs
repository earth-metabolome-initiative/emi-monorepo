impl From<
    crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::PpeReminderProcedureTemplate(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PpeReminderProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
