use crate::codegen::diesel_codegen::tables::{
    reagents::reagents, step_model_reagents::step_model_reagents,
};
diesel::allow_tables_to_appear_in_same_query!(step_model_reagents, reagents);
use crate::codegen::diesel_codegen::tables::step_models::step_models;
diesel::allow_tables_to_appear_in_same_query!(step_model_reagents, step_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(step_model_reagents, users);
