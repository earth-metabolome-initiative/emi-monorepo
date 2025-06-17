use crate::codegen::diesel_codegen::tables::{
    centrifugable_container_models::centrifugable_container_models,
    centrifuge_procedure_models::centrifuge_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(
    centrifuge_procedure_models,
    centrifugable_container_models
);
use crate::codegen::diesel_codegen::tables::centrifuge_models::centrifuge_models;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedure_models, centrifuge_models);
use crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables;
diesel::allow_tables_to_appear_in_same_query!(
    centrifuge_procedure_models,
    procedure_model_trackables
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(centrifuge_procedure_models, procedure_models);
use crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models;
diesel::allow_tables_to_appear_in_same_query!(
    centrifuge_procedure_models,
    volumetric_container_models
);
