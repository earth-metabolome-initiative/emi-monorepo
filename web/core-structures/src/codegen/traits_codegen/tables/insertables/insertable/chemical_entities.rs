#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableChemicalEntity;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableChemicalEntityBuilder;
}
