use crate::codegen::diesel_codegen::tables::rooms::rooms;
use crate::codegen::diesel_codegen::tables::addresses::addresses;
diesel::allow_tables_to_appear_in_same_query!(rooms, addresses);
use crate::codegen::diesel_codegen::tables::directus_users::directus_users;
diesel::allow_tables_to_appear_in_same_query!(rooms, directus_users);
use crate::codegen::diesel_codegen::tables::buildings::buildings;
diesel::allow_tables_to_appear_in_same_query!(rooms, buildings);
