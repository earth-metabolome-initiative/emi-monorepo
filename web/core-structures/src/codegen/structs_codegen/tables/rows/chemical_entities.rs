impl From<crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity>,
    ) -> Self {
        super::Rows::ChemicalEntity(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ChemicalEntity(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
