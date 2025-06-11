use crate::codegen::diesel_codegen::tables::{
    aliquoting_instrument_models::aliquoting_instrument_models,
    instrument_models::instrument_models,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_instrument_models, instrument_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_instrument_models, trackables);
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_instrument_models, commercial_products);
