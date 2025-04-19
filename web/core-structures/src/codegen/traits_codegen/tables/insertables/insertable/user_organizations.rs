#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder;
}
