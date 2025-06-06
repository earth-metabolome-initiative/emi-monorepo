use crate::codegen::diesel_codegen::tables::{
    freeze_drying_procedure_models::freeze_drying_procedure_models,
    procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(freeze_drying_procedure_models, procedure_models);
