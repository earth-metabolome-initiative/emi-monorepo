use crate::codegen::diesel_codegen::tables::{
    compatibility_rules::compatibility_rules, freezing_procedure_models::freezing_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, compatibility_rules);
use crate::codegen::diesel_codegen::tables::container_models::container_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, container_models);
use crate::codegen::diesel_codegen::tables::freezer_models::freezer_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, freezer_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    freezing_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, procedure_models);
