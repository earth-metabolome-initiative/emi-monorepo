impl From<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>,
    ) -> Self {
        super::Rows::PipetteTipModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::PipetteTipModel(v) => Some(v),
            _ => None,
        }
    }
}
