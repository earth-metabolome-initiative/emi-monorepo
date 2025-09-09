impl From<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::reagent_models::ReagentModel) -> Self {
        super::Row::ReagentModel(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::reagent_models::ReagentModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ReagentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
