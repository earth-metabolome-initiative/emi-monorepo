use crate::codegen::diesel_codegen::tables::{
    aliquoting_step_models::aliquoting_step_models, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(aliquoting_step_models, users);
use crate::codegen::diesel_codegen::tables::sampling_step_models::sampling_step_models;
diesel::allow_tables_to_appear_in_same_query!(aliquoting_step_models, sampling_step_models);
