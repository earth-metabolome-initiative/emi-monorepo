use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, reagent_models::reagent_models,
};
diesel::allow_tables_to_appear_in_same_query!(reagent_models, asset_models);
