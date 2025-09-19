impl From<
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
        >,
    ) -> Self {
        super::Rows::CommercialCentrifugeModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCentrifugeModel(v) => Some(v),
            _ => None,
        }
    }
}
