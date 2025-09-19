impl From<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine,
    ) -> Self {
        super::Row::BallMillMachine(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::BallMillMachine(v) => Some(v),
            _ => None,
        }
    }
}
