impl From<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep>,
    ) -> Self {
        super::Rows::BallMillStep(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
