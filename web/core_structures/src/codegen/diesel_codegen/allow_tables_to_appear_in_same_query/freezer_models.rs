use crate::codegen::diesel_codegen::tables::{
    freezer_models::freezer_models, trackables::trackables,
};
diesel::allow_tables_to_appear_in_same_query!(freezer_models, trackables);
