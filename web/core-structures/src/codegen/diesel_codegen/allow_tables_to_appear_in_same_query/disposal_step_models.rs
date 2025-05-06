use crate::codegen::diesel_codegen::tables::{
    disposal_step_models::disposal_step_models, step_models::step_models,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_step_models, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(disposal_step_models, users);
