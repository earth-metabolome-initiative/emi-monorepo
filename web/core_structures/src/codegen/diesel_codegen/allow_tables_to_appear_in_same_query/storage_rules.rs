use crate::codegen::diesel_codegen::tables::{
    container_models::container_models, storage_rules::storage_rules,
};
diesel::allow_tables_to_appear_in_same_query!(storage_rules, container_models);
