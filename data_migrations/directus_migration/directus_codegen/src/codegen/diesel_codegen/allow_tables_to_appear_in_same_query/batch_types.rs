use crate::codegen::diesel_codegen::tables::{
    batch_types::batch_types, directus_users::directus_users,
};
diesel::allow_tables_to_appear_in_same_query!(batch_types, directus_users);
