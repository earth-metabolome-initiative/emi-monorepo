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
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
