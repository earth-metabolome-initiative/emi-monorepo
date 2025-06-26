impl From<crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel,
    ) -> Self {
        super::Row::FreezeDrierModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::freeze_drier_models::FreezeDrierModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDrierModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
