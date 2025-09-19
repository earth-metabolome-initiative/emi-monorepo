impl From<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ) -> Self {
        super::Row::BallMillMachineModel(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::BallMillMachineModel(v) => Some(v),
            _ => None,
        }
    }
}
