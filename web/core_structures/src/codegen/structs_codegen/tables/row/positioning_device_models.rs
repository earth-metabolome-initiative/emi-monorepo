impl
    From<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    ) -> Self {
        super::Row::PositioningDeviceModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
