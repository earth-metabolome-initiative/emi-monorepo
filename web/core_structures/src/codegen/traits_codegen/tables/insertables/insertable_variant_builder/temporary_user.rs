#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder
{
    type Row = crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUser;
}
