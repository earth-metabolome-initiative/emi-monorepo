#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserEmailBuilder
{
    type Row = crate::codegen::structs_codegen::tables::temporary_user_emails::TemporaryUserEmail;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserEmail;
}
