use crate::codegen::diesel_codegen::tables::{directus_users::directus_users, rooms::rooms};
diesel::allow_tables_to_appear_in_same_query!(rooms, directus_users);
use crate::codegen::diesel_codegen::tables::buildings::buildings;
diesel::allow_tables_to_appear_in_same_query!(rooms, buildings);
use crate::codegen::diesel_codegen::tables::addresses::addresses;
diesel::allow_tables_to_appear_in_same_query!(rooms, addresses);
