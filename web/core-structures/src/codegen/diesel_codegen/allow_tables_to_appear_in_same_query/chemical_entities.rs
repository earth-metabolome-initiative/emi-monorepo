use crate::codegen::diesel_codegen::tables::{chemical_entities::chemical_entities, users::users};
diesel::allow_tables_to_appear_in_same_query!(chemical_entities, users);
