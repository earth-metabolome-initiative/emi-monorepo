use crate::codegen::diesel_codegen::tables::{
    pipette_models::pipette_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(pipette_models, trackables);
