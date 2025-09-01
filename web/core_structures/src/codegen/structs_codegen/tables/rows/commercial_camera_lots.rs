impl From<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
        >,
    ) -> Self {
        super::Rows::CommercialCameraLot(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCameraLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
