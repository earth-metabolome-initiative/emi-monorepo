use crate::codegen::diesel_codegen::tables::{
    tool_categories::tool_categories, tool_models::tool_models,
};
diesel::allow_tables_to_appear_in_same_query!(tool_models, tool_categories);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(tool_models, users);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(tool_models, commercial_products);
