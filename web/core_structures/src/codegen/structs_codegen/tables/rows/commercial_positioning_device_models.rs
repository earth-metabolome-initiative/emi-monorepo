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
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialPositioningDeviceModel(v) => Some(v),
            _ => None,
        }
    }
}
