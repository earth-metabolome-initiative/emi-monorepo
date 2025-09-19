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
impl From<super::Rows>
    for Option<
        Vec<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>,
    >
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::CommercialCameraLot(v) => Some(v),
            _ => None,
        }
    }
}
