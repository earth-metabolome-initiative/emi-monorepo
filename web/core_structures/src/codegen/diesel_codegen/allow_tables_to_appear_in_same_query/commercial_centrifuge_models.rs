use crate::codegen::diesel_codegen::tables::{
    centrifuge_models::centrifuge_models,
    commercial_centrifuge_models::commercial_centrifuge_models,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_models, centrifuge_models);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_models, commercial_products);
use crate::codegen::diesel_codegen::tables::asset_models::asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_models, asset_models);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_models, physical_asset_models);
