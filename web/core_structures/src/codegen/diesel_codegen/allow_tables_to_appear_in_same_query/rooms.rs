use crate::codegen::diesel_codegen::tables::{addresses::addresses, rooms::rooms};
diesel::allow_tables_to_appear_in_same_query!(rooms, addresses);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(rooms, users);
