use crate::codegen::diesel_codegen::tables::{
    centrifuge_models::centrifuge_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(centrifuge_models, trackables);
