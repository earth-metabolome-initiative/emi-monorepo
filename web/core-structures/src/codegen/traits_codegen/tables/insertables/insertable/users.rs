#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::users::User
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableUser;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder;
}
