#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::temporary_user_emails::TemporaryUserEmail
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserEmail;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserEmailBuilder;
}
