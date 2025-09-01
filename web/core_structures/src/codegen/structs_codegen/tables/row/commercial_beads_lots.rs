impl From<crate::codegen::structs_codegen::tables::commercial_beads_lots::CommercialBeadsLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_beads_lots::CommercialBeadsLot,
    ) -> Self {
        super::Row::CommercialBeadsLot(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_beads_lots::CommercialBeadsLot
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBeadsLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
