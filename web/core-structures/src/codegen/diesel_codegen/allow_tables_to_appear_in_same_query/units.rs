use crate::codegen::diesel_codegen::tables::{icons::icons, units::units};
diesel::allow_tables_to_appear_in_same_query!(units, icons);
use crate::codegen::diesel_codegen::tables::colors::colors;
diesel::allow_tables_to_appear_in_same_query!(units, colors);
