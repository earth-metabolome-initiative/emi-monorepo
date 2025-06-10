use crate::codegen::diesel_codegen::tables::colors::colors;
use crate::codegen::diesel_codegen::tables::roles::roles;
diesel::allow_tables_to_appear_in_same_query!(roles, colors);
