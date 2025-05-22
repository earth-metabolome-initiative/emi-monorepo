#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
{
    type Row = crate::codegen::structs_codegen::tables::users::User;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableUser;
}
