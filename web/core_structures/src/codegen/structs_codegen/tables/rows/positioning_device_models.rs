impl
    From<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        >,
    ) -> Self {
        super::Rows::PositioningDeviceModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PositioningDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
