#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganizationBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organizations::Organization;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableOrganization;
}
