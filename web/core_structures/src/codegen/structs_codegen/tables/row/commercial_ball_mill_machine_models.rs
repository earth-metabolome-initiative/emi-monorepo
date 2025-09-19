impl From<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
    ) -> Self {
        super::Row::CommercialBallMillMachineModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialBallMillMachineModel(v) => Some(v),
            _ => None,
        }
    }
}
