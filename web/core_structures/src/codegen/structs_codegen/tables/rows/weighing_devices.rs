impl From<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>,
    ) -> Self {
        super::Rows::WeighingDevice(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::WeighingDevice(v) => Some(v),
            _ => None,
        }
    }
}
