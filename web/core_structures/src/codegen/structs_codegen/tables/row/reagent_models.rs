impl From<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel) -> Self {
        super::Row::ReagentModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::ReagentModel(v) => Some(v),
            _ => None,
        }
    }
}
