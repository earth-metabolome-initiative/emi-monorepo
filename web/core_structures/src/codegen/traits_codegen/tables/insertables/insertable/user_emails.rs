#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::user_emails::UserEmail
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder;
}
