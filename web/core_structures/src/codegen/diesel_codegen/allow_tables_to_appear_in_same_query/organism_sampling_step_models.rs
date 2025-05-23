use crate::codegen::diesel_codegen::tables::{
    organism_sampling_step_models::organism_sampling_step_models, organisms::organisms,
};
diesel::allow_tables_to_appear_in_same_query!(organism_sampling_step_models, organisms);
use crate::codegen::diesel_codegen::tables::sampling_step_models::sampling_step_models;
diesel::allow_tables_to_appear_in_same_query!(organism_sampling_step_models, sampling_step_models);
