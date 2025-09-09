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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
