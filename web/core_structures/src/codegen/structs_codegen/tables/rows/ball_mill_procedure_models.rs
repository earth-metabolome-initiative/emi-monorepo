impl
    From<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    > for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
        >,
    ) -> Self {
        super::Rows::BallMillProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
