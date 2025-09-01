impl From<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>,
    ) -> Self {
        super::Rows::ReagentModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
