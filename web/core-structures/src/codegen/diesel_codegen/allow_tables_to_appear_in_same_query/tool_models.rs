use crate::codegen::diesel_codegen::tables::{tool_models::tool_models, users::users};
diesel::allow_tables_to_appear_in_same_query!(tool_models, users);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(tool_models, commercial_products);
use crate::codegen::diesel_codegen::tables::tool_categories::tool_categories;
diesel::allow_tables_to_appear_in_same_query!(tool_models, tool_categories);
