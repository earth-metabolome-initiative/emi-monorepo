use crate::codegen::diesel_codegen::tables::{
    compatibility_rules::compatibility_rules, storage_procedure_models::storage_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_models, compatibility_rules);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_models, container_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_models, procedure_model_trackables);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(storage_procedure_models, procedure_models);
