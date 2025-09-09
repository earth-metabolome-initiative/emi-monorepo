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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
