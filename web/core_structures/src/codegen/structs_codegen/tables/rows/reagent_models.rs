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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::ReagentModel(v) => Some(v),
            _ => None,
        }
    }
}
