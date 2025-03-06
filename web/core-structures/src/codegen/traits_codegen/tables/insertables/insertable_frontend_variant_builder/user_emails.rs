#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder {
    type Row = crate::codegen::structs_codegen::tables::user_emails::UserEmail;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableUserEmail;
}
