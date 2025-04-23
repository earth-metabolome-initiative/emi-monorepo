use crate::codegen::diesel_codegen::tables::{
    procedure_model_tool_categories::procedure_model_tool_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_model_tool_categories, users);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(procedure_model_tool_categories, procedure_models);
use crate::codegen::diesel_codegen::tables::tool_categories::tool_categories;
diesel::allow_tables_to_appear_in_same_query!(procedure_model_tool_categories, tool_categories);
