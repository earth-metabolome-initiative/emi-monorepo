impl web_common_traits::database::Insertable for crate::Organism {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganism;
}
