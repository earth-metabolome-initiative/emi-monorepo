use crate::codegen::diesel_codegen::tables::{addresses::addresses, countries::countries};
diesel::allow_tables_to_appear_in_same_query!(addresses, countries);
use crate::codegen::diesel_codegen::tables::cities::cities;
diesel::allow_tables_to_appear_in_same_query!(addresses, cities);
