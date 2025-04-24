use crate::codegen::diesel_codegen::tables::{photographs::photographs, step_models::step_models};
diesel::allow_tables_to_appear_in_same_query!(step_models, photographs);
use crate::codegen::diesel_codegen::tables::step_model_categories::step_model_categories;
diesel::allow_tables_to_appear_in_same_query!(step_models, step_model_categories);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_models, users);
