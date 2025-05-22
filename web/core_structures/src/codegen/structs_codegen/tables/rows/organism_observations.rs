impl From<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>>
    for super::Rows
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    ) -> Self {
        super::Rows::OrganismObservation(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::OrganismObservation(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
