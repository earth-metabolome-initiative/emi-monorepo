impl From<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>,
    ) -> Self {
        super::Rows::PositioningDevice(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PositioningDevice(v) => Some(v),
            _ => None,
        }
    }
}
