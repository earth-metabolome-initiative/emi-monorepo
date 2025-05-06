impl From<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep) -> Self {
        super::Row::BallMillStep(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
