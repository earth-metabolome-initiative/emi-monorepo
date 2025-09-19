impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    ) -> Self {
        super::Row::CommercialWeighingDeviceModel(value)
    }
}
impl From<super::Row>
for Option<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialWeighingDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
