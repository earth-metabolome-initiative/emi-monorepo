impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::materials::Material
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableMaterial;
}
