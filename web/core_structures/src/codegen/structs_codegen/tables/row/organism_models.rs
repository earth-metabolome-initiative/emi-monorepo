impl From<crate::codegen::structs_codegen::tables::organism_models::OrganismModel> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    ) -> Self {
        super::Row::OrganismModel(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::OrganismModel(v) => Some(v),
            _ => None,
        }
    }
}
