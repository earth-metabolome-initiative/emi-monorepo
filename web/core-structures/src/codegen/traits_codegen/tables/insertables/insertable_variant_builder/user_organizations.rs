#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganization;
}
