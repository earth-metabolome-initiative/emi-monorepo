impl From<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
    ) -> Self {
        super::Row::FreezeDryerModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
