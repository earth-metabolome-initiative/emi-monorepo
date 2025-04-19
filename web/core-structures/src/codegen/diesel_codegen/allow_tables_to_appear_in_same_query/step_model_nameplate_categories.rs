use crate::codegen::diesel_codegen::tables::{
    step_model_nameplate_categories::step_model_nameplate_categories, step_models::step_models,
};
diesel::allow_tables_to_appear_in_same_query!(step_model_nameplate_categories, step_models);
use crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories;
diesel::allow_tables_to_appear_in_same_query!(
    step_model_nameplate_categories,
    nameplate_categories
);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_model_nameplate_categories, users);
