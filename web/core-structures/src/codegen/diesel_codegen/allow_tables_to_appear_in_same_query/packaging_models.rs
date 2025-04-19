use crate::codegen::diesel_codegen::tables::{packaging_models::packaging_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(packaging_models, users);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(packaging_models, commercial_products);
