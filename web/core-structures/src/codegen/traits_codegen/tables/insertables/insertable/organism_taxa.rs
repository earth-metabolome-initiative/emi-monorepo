#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxa
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxa;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxaBuilder;
}
