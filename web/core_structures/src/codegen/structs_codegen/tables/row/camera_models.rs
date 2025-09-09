impl From<crate::codegen::structs_codegen::tables::camera_models::CameraModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::camera_models::CameraModel) -> Self {
        super::Row::CameraModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::camera_models::CameraModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
