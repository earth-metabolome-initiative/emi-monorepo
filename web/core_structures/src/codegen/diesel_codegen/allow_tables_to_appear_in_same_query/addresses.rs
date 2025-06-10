use crate::codegen::diesel_codegen::tables::addresses::addresses;
use crate::codegen::diesel_codegen::tables::cities::cities;
diesel::allow_tables_to_appear_in_same_query!(addresses, cities);
