use crate::codegen::diesel_codegen::tables::parent_procedure_models::parent_procedure_models;
use crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_model_trackables,
    parent_procedure_models
);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_model_trackables,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_model_trackables, procedure_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_model_trackables, trackables);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_model_trackables, users);
