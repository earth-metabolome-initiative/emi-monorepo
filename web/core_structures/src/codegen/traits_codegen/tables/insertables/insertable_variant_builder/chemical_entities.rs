#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableChemicalEntityBuilder
{
    type Row = crate::codegen::structs_codegen::tables::chemical_entities::ChemicalEntity;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableChemicalEntity;
}
