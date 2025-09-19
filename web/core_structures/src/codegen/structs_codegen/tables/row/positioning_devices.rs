impl From<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice,
    ) -> Self {
        super::Row::PositioningDevice(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PositioningDevice(v) => Some(v),
            _ => None,
        }
    }
}
