impl From<crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    ) -> Self {
        super::Row::CommercialCameraModel(value)
    }
}
impl From<super::Row>
    for Option<
        crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    >
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCameraModel(v) => Some(v),
            _ => None,
        }
    }
}
