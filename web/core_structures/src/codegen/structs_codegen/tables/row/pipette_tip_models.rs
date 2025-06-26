impl From<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    ) -> Self {
        super::Row::PipetteTipModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PipetteTipModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
