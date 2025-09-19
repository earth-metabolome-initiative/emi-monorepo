impl From<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    ) -> Self {
        super::Row::WeighingDevice(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::WeighingDevice(v) => Some(v),
            _ => None,
        }
    }
}
