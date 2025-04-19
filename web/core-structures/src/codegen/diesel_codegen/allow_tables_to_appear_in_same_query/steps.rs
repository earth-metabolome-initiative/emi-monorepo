use crate::codegen::diesel_codegen::tables::{step_models::step_models, steps::steps};
diesel::allow_tables_to_appear_in_same_query!(steps, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(steps, users);
use crate::codegen::diesel_codegen::tables::procedures::procedures;
diesel::allow_tables_to_appear_in_same_query!(steps, procedures);
