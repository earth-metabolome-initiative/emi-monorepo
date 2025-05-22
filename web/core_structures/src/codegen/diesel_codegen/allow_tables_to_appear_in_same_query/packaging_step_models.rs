use crate::codegen::diesel_codegen::tables::{
    packaging_models::packaging_models, packaging_step_models::packaging_step_models,
};
diesel::allow_tables_to_appear_in_same_query!(packaging_step_models, packaging_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(packaging_step_models, users);
