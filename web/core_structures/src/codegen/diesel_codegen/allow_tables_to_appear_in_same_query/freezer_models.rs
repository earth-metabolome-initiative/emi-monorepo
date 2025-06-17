use crate::codegen::diesel_codegen::tables::{
    freezer_models::freezer_models, instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(freezer_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(freezer_models, trackables);
