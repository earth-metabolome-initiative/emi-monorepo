#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon;
}
