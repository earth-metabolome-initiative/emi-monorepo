impl From<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    ) -> Self {
        super::Row::FreezeDryerModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::FreezeDryerModel(v) => Some(v),
            _ => None,
        }
    }
}
