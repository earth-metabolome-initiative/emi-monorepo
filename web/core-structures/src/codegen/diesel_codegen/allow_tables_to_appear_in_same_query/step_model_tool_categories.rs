use crate::codegen::diesel_codegen::tables::{
    step_model_tool_categories::step_model_tool_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(step_model_tool_categories, users);
use crate::codegen::diesel_codegen::tables::tool_categories::tool_categories;
diesel::allow_tables_to_appear_in_same_query!(step_model_tool_categories, tool_categories);
use crate::codegen::diesel_codegen::tables::step_models::step_models;
diesel::allow_tables_to_appear_in_same_query!(step_model_tool_categories, step_models);
