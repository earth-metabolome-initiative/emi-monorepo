impl From<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl
    From<
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    > for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    ) -> Self {
        super::Rows::BallMillMachineModel(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::BallMillMachineModel(v) => Some(v),
            _ => None,
        }
    }
}
