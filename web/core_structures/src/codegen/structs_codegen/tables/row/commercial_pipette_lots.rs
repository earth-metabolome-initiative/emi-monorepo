impl From<crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    ) -> Self {
        super::Row::CommercialPipetteLot(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
