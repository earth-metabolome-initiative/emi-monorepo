impl From<crate::codegen::structs_codegen::tables::camera_models::CameraModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::camera_models::CameraModel) -> Self {
        super::Row::CameraModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::camera_models::CameraModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CameraModel(v) => Some(v),
            _ => None,
        }
    }
}
