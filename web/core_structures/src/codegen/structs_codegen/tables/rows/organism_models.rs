impl From<crate::codegen::structs_codegen::tables::organism_models::OrganismModel> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>,
    ) -> Self {
        super::Rows::OrganismModel(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::OrganismModel(v) => Some(v),
            _ => None,
        }
    }
}
