impl From<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>,
    ) -> Self {
        super::Rows::BallMillMachine(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::BallMillMachine(v) => Some(v),
            _ => None,
        }
    }
}
