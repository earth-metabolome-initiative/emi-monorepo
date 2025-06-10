use crate::codegen::diesel_codegen::tables::colors::colors;
use crate::codegen::diesel_codegen::tables::materials::materials;
diesel::allow_tables_to_appear_in_same_query!(materials, colors);
