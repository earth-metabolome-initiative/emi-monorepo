use crate::codegen::diesel_codegen::tables::{
    aliquoting_instrument_models::aliquoting_instrument_models,
    instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_instrument_models, instrument_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_instrument_models, users);
