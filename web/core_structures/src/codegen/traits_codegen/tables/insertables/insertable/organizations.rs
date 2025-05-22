#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::organizations::Organization
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganization;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder;
}
