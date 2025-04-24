use crate::codegen::diesel_codegen::tables::{icons::icons, roles::roles};
diesel::allow_tables_to_appear_in_same_query!(roles, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(roles, colors);
