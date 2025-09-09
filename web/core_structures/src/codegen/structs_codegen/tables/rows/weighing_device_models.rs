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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
