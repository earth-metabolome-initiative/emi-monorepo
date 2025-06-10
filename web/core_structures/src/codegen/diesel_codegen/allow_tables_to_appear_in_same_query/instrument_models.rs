use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
use crate::codegen::diesel_codegen::tables::instrument_models::instrument_models;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, commercial_products);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(instrument_models, trackables);
