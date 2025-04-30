use crate::codegen::diesel_codegen::tables::document_formats::document_formats;
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(document_formats, icons);
