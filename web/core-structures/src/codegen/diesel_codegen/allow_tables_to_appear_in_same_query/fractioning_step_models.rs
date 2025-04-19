use crate::codegen::diesel_codegen::tables::{
    fractioning_step_models::fractioning_step_models, step_models::step_models,
};
diesel::allow_tables_to_appear_in_same_query!(fractioning_step_models, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(fractioning_step_models, users);
use crate::codegen::diesel_codegen::tables::step_model_instrument_categories::step_model_instrument_categories;
diesel::allow_tables_to_appear_in_same_query!(
    fractioning_step_models,
    step_model_instrument_categories
);
