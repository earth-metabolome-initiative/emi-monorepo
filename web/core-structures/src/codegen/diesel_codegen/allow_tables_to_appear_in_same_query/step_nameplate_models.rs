use crate::codegen::diesel_codegen::tables::{
    step_nameplate_models::step_nameplate_models, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(step_nameplate_models, users);
use crate::codegen::diesel_codegen::tables::nameplate_models::nameplate_models;
diesel::allow_tables_to_appear_in_same_query!(step_nameplate_models, nameplate_models);
use crate::codegen::diesel_codegen::tables::steps::steps;
diesel::allow_tables_to_appear_in_same_query!(step_nameplate_models, steps);
