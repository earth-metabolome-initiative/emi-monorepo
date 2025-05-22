use crate::codegen::diesel_codegen::tables::{step_tool_models::step_tool_models, steps::steps};
diesel::allow_tables_to_appear_in_same_query!(step_tool_models, steps);
use crate::codegen::diesel_codegen::tables::tool_models::tool_models;
diesel::allow_tables_to_appear_in_same_query!(step_tool_models, tool_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_tool_models, users);
