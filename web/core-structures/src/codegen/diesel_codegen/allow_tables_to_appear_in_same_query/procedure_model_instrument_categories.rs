use crate::codegen::diesel_codegen::tables::{
    procedure_model_instrument_categories::procedure_model_instrument_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_model_instrument_categories, users);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(
    procedure_model_instrument_categories,
    procedure_models
);
use crate::codegen::diesel_codegen::tables::instrument_categories::instrument_categories;
diesel::allow_tables_to_appear_in_same_query!(
    procedure_model_instrument_categories,
    instrument_categories
);
