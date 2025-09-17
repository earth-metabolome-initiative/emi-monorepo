impl From<
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
    ) -> Self {
        super::Row::CommercialBallMillMachineLot(value)
    }
}
impl TryFrom<super::Row>
for crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBallMillMachineLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
