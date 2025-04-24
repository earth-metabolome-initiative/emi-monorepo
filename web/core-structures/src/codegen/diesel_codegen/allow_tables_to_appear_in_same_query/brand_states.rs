use crate::codegen::diesel_codegen::tables::{brand_states::brand_states, colors::colors};
diesel::allow_tables_to_appear_in_same_query!(brand_states, colors);
use crate::codegen::diesel_codegen::tables::icons::icons;
diesel::allow_tables_to_appear_in_same_query!(brand_states, icons);
