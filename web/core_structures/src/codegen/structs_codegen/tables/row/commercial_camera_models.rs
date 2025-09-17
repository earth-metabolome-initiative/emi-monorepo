impl From<crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
    ) -> Self {
        super::Row::CommercialCameraModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCameraModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
