use crate::codegen::diesel_codegen::tables::{
    asset_model_ancestors::asset_model_ancestors, asset_models::asset_models,
};
diesel::allow_tables_to_appear_in_same_query!(asset_model_ancestors, asset_models);
