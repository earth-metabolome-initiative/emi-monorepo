use crate::codegen::diesel_codegen::tables::{
    disposal_step_models::disposal_step_models, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(disposal_step_models, users);
use crate::codegen::diesel_codegen::tables::step_models::step_models;
diesel::allow_tables_to_appear_in_same_query!(disposal_step_models, step_models);
