impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    ) -> Self {
        super::Row::CommercialCentrifugeLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCentrifugeLot(v) => Some(v),
            _ => None,
        }
    }
}
