impl From<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
        >,
    ) -> Self {
        super::Rows::CommercialBeadLot(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBeadLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
