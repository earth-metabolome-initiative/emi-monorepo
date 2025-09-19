impl From<crate::codegen::structs_codegen::tables::camera_models::CameraModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::camera_models::CameraModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>,
    ) -> Self {
        super::Rows::CameraModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CameraModel(v) => Some(v),
            _ => None,
        }
    }
}
