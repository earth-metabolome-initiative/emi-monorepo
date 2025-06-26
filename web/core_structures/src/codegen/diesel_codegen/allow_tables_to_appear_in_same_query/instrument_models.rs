use crate::codegen::diesel_codegen::tables::{
    instrument_models::instrument_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(instrument_models, trackables);
