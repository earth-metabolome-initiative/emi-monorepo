impl From<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
    ) -> Self {
        super::Row::CommercialBallMillMachineLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialBallMillMachineLot(v) => Some(v),
            _ => None,
        }
    }
}
