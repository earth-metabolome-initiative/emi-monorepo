impl From<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
    ) -> Self {
        super::Row::CommercialCameraLot(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::CommercialCameraLot(v) => Some(v),
            _ => None,
        }
    }
}
