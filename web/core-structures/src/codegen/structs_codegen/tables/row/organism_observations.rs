impl From<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
    ) -> Self {
        super::Row::OrganismObservation(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::OrganismObservation(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
