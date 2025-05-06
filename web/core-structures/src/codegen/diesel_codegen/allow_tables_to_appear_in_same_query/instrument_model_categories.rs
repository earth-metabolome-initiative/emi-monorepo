use crate::codegen::diesel_codegen::tables::{
    instrument_model_categories::instrument_model_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(instrument_model_categories, users);
use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
diesel::allow_tables_to_appear_in_same_query!(instrument_model_categories, instrument_models);
