use crate::codegen::diesel_codegen::tables::{
    grinding_step_models::grinding_step_models,
    step_model_instrument_categories::step_model_instrument_categories,
};
diesel::allow_tables_to_appear_in_same_query!(
    grinding_step_models,
    step_model_instrument_categories
);
use crate::codegen::diesel_codegen::tables::step_models::step_models;
diesel::allow_tables_to_appear_in_same_query!(grinding_step_models, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(grinding_step_models, users);
