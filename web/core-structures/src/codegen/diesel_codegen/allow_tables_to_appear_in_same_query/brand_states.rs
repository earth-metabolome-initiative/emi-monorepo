use crate::codegen::diesel_codegen::tables::{brand_states::brand_states, icons::icons};
diesel::allow_tables_to_appear_in_same_query!(brand_states, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(brand_states, colors);
