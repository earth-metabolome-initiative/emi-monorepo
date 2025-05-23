use crate::codegen::diesel_codegen::tables::{
    step_models::step_models, weighing_step_models::weighing_step_models,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_step_models, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(weighing_step_models, users);
