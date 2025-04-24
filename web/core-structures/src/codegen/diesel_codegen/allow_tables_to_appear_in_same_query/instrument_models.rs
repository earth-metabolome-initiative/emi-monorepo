use crate::codegen::diesel_codegen::tables::{instrument_models::instrument_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(instrument_models, users);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, commercial_products);
