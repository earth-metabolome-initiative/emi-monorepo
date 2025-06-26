use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, pipette_models::pipette_models,
};
diesel::allow_tables_to_appear_in_same_query!(pipette_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(pipette_models, trackables);
