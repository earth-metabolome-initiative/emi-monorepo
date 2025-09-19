impl From<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    ) -> Self {
        super::Row::BallMillProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::BallMillProcedure(v) => Some(v),
            _ => None,
        }
    }
}
