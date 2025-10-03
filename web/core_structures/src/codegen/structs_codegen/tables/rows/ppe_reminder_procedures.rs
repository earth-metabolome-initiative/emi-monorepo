impl From<crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure>,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure,
        >,
    ) -> Self {
        super::Rows::PpeReminderProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PpeReminderProcedure(v) => Some(v),
            _ => None,
        }
    }
}
