use crate::codegen::diesel_codegen::tables::{
    step_model_instrument_categories::step_model_instrument_categories,
    step_model_instrument_models::step_model_instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    step_model_instrument_models,
    step_model_instrument_categories
);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_model_instrument_models, users);
use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
diesel::allow_tables_to_appear_in_same_query!(step_model_instrument_models, instrument_models);
