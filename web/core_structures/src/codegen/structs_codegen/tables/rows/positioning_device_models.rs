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
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
