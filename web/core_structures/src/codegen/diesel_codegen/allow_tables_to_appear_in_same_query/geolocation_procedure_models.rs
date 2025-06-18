use crate::codegen::diesel_codegen::tables::{
    geolocation_procedure_models::geolocation_procedure_models,
    positioning_device_models::positioning_device_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    geolocation_procedure_models,
    positioning_device_models
);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    geolocation_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(geolocation_procedure_models, procedure_models);
