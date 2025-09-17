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
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::organism_models::OrganismModel>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::OrganismModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
