impl From<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    ) -> Self {
        super::Row::BallMillMachineModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::BallMillMachineModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
