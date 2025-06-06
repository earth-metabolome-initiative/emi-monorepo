impl
    From<
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    > for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    ) -> Self {
        super::Row::BallMillProcedureModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
