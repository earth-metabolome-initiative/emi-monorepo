impl From<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        >,
    ) -> Self {
        super::Rows::WeighingDeviceModel(value)
    }
}
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::WeighingDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
