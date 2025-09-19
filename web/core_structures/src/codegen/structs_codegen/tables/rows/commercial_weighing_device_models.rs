impl From<
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
        >,
    ) -> Self {
        super::Rows::CommercialWeighingDeviceModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialWeighingDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
