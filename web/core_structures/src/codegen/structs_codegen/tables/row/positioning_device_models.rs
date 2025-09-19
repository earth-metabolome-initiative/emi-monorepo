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
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PositioningDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
