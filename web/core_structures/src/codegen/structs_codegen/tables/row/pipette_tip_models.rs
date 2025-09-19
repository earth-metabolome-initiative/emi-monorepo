impl From<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    ) -> Self {
        super::Row::PipetteTipModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::PipetteTipModel(v) => Some(v),
            _ => None,
        }
    }
}
