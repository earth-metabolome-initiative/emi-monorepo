#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableNameplateModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableNameplateModel;
}
