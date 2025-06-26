use crate::codegen::diesel_codegen::tables::{
    freezer_models::freezer_models, freezing_procedure_models::freezing_procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, freezer_models);
use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, storage_procedure_models);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(freezing_procedure_models, procedure_models);
