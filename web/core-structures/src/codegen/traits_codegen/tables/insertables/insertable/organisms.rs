#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganism;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder;
}
