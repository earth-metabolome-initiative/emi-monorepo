impl From<crate::codegen::structs_codegen::tables::organism_models::OrganismModel> for super::Row {
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    ) -> Self {
        super::Row::OrganismModel(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::organism_models::OrganismModel
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::OrganismModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
