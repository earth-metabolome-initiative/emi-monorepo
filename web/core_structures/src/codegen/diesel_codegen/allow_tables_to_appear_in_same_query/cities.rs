use crate::codegen::diesel_codegen::tables::{cities::cities, countries::countries};
diesel::allow_tables_to_appear_in_same_query!(cities, countries);
