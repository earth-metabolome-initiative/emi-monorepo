#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organisms::Organism;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableOrganism;
}
