use crate::codegen::diesel_codegen::tables::cities::cities;
use crate::codegen::diesel_codegen::tables::countries::countries;
diesel::allow_tables_to_appear_in_same_query!(cities, countries);
