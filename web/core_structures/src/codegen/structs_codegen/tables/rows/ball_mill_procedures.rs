impl From<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
        >,
    ) -> Self {
        super::Rows::BallMillProcedure(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::BallMillProcedure(v) => Some(v),
            _ => None,
        }
    }
}
