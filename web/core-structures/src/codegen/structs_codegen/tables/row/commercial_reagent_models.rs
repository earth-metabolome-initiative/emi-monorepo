impl
    From<crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    ) -> Self {
        super::Row::CommercialReagentModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
