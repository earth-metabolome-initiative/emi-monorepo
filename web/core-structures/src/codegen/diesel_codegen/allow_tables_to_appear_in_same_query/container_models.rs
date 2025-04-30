use crate::codegen::diesel_codegen::tables::{container_models::container_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(container_models, users);
use crate::codegen::diesel_codegen::tables::container_categories::container_categories;
diesel::allow_tables_to_appear_in_same_query!(container_models, container_categories);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(container_models, commercial_products);
