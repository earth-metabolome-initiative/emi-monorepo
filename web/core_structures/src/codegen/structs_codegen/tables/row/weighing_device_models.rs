impl From<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    ) -> Self {
        super::Row::WeighingDeviceModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::WeighingDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
