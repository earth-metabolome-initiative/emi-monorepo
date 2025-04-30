use crate::codegen::diesel_codegen::tables::{
    procedure_models::procedure_models, procedures::procedures,
};
diesel::allow_tables_to_appear_in_same_query!(procedures, procedure_models);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(procedures, users);
