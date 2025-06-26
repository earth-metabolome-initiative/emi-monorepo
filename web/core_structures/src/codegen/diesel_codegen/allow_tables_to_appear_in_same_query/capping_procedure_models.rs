use crate::codegen::diesel_codegen::tables::{
    capping_procedure_models::capping_procedure_models, compatibility_rules::compatibility_rules,
};
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, compatibility_rules);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, procedure_model_trackables);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, procedure_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(capping_procedure_models, trackables);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    capping_procedure_models,
    volumetric_container_models
);
