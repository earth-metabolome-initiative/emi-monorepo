impl
    From<crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
        >,
    ) -> Self {
        super::Rows::CommercialReagentModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
