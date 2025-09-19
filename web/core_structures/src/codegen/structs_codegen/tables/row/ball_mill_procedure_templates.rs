impl From<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    ) -> Self {
        super::Row::BallMillProcedureTemplate(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::BallMillProcedureTemplate(v) => Some(v),
            _ => None,
        }
    }
}
