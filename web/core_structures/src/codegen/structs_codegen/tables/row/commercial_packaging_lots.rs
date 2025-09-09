impl
    From<crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    ) -> Self {
        super::Row::CommercialPackagingLot(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPackagingLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
