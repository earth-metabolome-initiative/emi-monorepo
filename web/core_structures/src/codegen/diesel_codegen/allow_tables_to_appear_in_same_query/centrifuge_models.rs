use crate::codegen::diesel_codegen::tables::{
    centrifuge_models::centrifuge_models, instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(centrifuge_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_models, trackables);
