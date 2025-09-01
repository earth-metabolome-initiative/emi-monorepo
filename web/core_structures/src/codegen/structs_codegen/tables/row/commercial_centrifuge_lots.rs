impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    ) -> Self {
        super::Row::CommercialCentrifugeLot(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCentrifugeLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
