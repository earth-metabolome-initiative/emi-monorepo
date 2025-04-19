use crate::codegen::diesel_codegen::tables::{
    users::users, weighing_instrument_models::weighing_instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_instrument_models, users);
use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
diesel::allow_tables_to_appear_in_same_query!(weighing_instrument_models, instrument_models);
