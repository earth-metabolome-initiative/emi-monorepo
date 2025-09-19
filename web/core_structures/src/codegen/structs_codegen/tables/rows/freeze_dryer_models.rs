impl From<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>,
    ) -> Self {
        super::Rows::FreezeDryerModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezeDryerModel(v) => Some(v),
            _ => None,
        }
    }
}
