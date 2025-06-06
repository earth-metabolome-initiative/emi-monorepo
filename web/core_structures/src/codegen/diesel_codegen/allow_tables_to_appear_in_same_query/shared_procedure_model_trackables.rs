use crate::codegen::diesel_codegen::tables::{
    procedure_model_trackables::procedure_model_trackables,
    shared_procedure_model_trackables::shared_procedure_model_trackables,
};
diesel::allow_tables_to_appear_in_same_query!(
    shared_procedure_model_trackables,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(shared_procedure_model_trackables, users);
