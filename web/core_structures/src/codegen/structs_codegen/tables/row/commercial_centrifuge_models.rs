impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    ) -> Self {
        super::Row::CommercialCentrifugeModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCentrifugeModel(v) => Some(v),
            _ => None,
        }
    }
}
