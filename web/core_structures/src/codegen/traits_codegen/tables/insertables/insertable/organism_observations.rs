#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismObservation;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismObservationBuilder;
}
