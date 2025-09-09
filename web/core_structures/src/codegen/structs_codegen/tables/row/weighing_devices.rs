impl From<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    ) -> Self {
        super::Row::WeighingDevice(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::WeighingDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
