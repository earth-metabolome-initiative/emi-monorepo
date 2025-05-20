#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder;
}
