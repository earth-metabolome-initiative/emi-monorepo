use crate::codegen::diesel_codegen::tables::{
    next_procedure_templates::next_procedure_templates, procedure_templates::procedure_templates,
};
diesel::allow_tables_to_appear_in_same_query!(next_procedure_templates, procedure_templates);
use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
diesel::allow_tables_to_appear_in_same_query!(next_procedure_templates, parent_procedure_templates);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(next_procedure_templates, users);
