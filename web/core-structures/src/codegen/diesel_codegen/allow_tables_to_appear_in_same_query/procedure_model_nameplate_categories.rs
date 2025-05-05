use crate::codegen::diesel_codegen::tables::{
    procedure_model_nameplate_categories::procedure_model_nameplate_categories, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_model_nameplate_categories, users);
use crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories;
diesel::allow_tables_to_appear_in_same_query!(
    procedure_model_nameplate_categories,
    nameplate_categories
);
use crate::codegen::diesel_codegen::tables::procedure_models::procedure_models;
diesel::allow_tables_to_appear_in_same_query!(
    procedure_model_nameplate_categories,
    procedure_models
);
