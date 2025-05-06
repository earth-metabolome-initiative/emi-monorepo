use crate::codegen::diesel_codegen::tables::brand_states::brand_states;
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(brand_states, colors);
