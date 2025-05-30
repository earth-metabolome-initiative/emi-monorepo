impl From<crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity,
    ) -> Self {
        super::Row::ChemicalEntity(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ChemicalEntity(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
