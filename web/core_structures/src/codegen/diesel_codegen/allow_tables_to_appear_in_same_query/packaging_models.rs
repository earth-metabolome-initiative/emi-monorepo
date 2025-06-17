use crate::codegen::diesel_codegen::tables::{
    materials::materials, packaging_models::packaging_models,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_models, materials);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(packaging_models, trackables);
