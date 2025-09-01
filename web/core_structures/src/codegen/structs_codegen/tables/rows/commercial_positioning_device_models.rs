impl From<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
        >,
    ) -> Self {
        super::Rows::CommercialPositioningDeviceModel(value)
    }
}
impl TryFrom<super::Rows>
for Vec<
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialPositioningDeviceModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
