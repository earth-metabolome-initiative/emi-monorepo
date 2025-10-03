impl From<crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure,
    ) -> Self {
        super::Row::PpeReminderProcedure(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PpeReminderProcedure(v) => Some(v),
            _ => None,
        }
    }
}
