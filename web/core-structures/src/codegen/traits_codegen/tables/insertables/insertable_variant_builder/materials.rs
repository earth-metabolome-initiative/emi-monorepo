#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
{
    type Row = crate::codegen::structs_codegen::tables::materials::Material;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableMaterial;
}
