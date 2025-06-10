use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
use crate::codegen::diesel_codegen::tables::packaging_models::packaging_models;
diesel::allow_tables_to_appear_in_same_query!(packaging_models, commercial_products);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(packaging_models, trackables);
