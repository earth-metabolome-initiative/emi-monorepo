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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PositioningDevice(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
