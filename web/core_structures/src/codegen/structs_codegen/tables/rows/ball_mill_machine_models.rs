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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::BallMillMachineModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
