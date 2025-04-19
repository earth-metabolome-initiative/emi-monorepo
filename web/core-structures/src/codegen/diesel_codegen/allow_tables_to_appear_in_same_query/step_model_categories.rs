use crate::codegen::diesel_codegen::tables::{
    icons::icons, step_model_categories::step_model_categories,
};
diesel::allow_tables_to_appear_in_same_query!(step_model_categories, icons);
