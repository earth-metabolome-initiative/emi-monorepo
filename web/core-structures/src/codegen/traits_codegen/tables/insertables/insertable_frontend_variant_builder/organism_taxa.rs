#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxaBuilder {
    type Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxa;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxa;
}
