impl From<
    crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
    ) -> Self {
        super::Row::PpeReminderProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PpeReminderProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
