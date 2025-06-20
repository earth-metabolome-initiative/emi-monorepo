use crate::codegen::diesel_codegen::tables::{
    pipette_tip_models::pipette_tip_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(pipette_tip_models, trackables);
