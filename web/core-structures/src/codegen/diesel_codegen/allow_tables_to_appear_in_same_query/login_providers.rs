use crate::codegen::diesel_codegen::tables::{icons::icons, login_providers::login_providers};
diesel::allow_tables_to_appear_in_same_query!(login_providers, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(login_providers, colors);
