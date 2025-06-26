use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, weighing_instrument_models::weighing_instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(weighing_instrument_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(weighing_instrument_models, trackables);
