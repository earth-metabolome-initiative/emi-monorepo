#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder;
}
