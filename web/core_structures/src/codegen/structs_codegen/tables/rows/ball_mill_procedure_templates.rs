impl From<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
        >,
    ) -> Self {
        super::Rows::BallMillProcedureTemplate(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillProcedureTemplate(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
