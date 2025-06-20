use crate::codegen::diesel_codegen::tables::{
    procedure_model_trackables::procedure_model_trackables, procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_model_trackables, procedure_models);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(procedure_model_trackables, trackables);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedure_model_trackables, users);
