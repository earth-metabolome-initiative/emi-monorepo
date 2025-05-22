impl From<crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel,
    ) -> Self {
        super::Row::BallMillStepModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillStepModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
