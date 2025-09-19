impl From<crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
        >,
    ) -> Self {
        super::Rows::CommercialCameraModel(value)
    }
}
impl From<super::Rows>
for Option<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    >,
> {
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCameraModel(v) => Some(v),
            _ => None,
        }
    }
}
