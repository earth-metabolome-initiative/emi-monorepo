use crate::codegen::diesel_codegen::tables::{
    procedure_templates::procedure_templates, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(procedure_templates, users);
