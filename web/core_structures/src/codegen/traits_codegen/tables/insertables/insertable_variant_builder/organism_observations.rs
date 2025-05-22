#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismObservationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismObservation;
}
