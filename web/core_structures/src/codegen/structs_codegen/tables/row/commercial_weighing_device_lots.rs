impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
    ) -> Self {
        super::Row::CommercialWeighingDeviceLot(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialWeighingDeviceLot(v) => Some(v),
            _ => None,
        }
    }
}
