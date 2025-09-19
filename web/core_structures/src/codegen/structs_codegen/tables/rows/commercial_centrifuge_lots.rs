impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
        >,
    ) -> Self {
        super::Rows::CommercialCentrifugeLot(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCentrifugeLot(v) => Some(v),
            _ => None,
        }
    }
}
