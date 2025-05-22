use crate::codegen::diesel_codegen::tables::{
    project_workflow_models::project_workflow_models, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(project_workflow_models, users);
