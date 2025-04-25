use crate::codegen::diesel_codegen::tables::{icons::icons, materials::materials};
diesel::allow_tables_to_appear_in_same_query!(materials, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(materials, colors);
