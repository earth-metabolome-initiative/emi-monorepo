#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableNameplateModelBuilder;
}
