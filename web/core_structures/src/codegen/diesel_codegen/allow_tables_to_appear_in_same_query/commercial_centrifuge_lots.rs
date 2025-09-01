use crate::codegen::diesel_codegen::tables::{
    asset_models::asset_models, commercial_centrifuge_lots::commercial_centrifuge_lots,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_lots, asset_models);
use crate::codegen::diesel_codegen::tables::centrifuge_models::centrifuge_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_lots, centrifuge_models);
use crate::codegen::diesel_codegen::tables::commercial_centrifuge_models::commercial_centrifuge_models;
diesel::allow_tables_to_appear_in_same_query!(
    commercial_centrifuge_lots,
    commercial_centrifuge_models
);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_lots, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models;
diesel::allow_tables_to_appear_in_same_query!(commercial_centrifuge_lots, physical_asset_models);
