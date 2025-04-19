#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableRoleBuilder
{
    type Row = crate::codegen::structs_codegen::tables::roles::Role;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableRole;
}
