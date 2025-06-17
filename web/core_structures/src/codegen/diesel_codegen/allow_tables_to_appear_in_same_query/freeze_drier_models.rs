use crate::codegen::diesel_codegen::tables::{
    freeze_drier_models::freeze_drier_models, instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(freeze_drier_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(freeze_drier_models, trackables);
