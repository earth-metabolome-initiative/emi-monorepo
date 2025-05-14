use crate::codegen::diesel_codegen::tables::{rooms::rooms, users::users};
diesel::allow_tables_to_appear_in_same_query!(rooms, users);
use crate::codegen::diesel_codegen::tables::addresses::addresses;
diesel::allow_tables_to_appear_in_same_query!(rooms, addresses);
