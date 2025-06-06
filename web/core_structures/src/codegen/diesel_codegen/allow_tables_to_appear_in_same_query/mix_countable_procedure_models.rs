use crate::codegen::diesel_codegen::tables::{
    mix_countable_procedure_models::mix_countable_procedure_models,
    procedure_models::procedure_models,
};
diesel::allow_tables_to_appear_in_same_query!(mix_countable_procedure_models, procedure_models);
