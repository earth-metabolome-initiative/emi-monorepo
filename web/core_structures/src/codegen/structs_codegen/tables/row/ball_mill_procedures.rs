impl From<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    ) -> Self {
        super::Row::BallMillProcedure(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillProcedure(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
