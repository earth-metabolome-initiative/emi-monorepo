use crate::codegen::diesel_codegen::tables::{colors::colors, units::units};
diesel::allow_tables_to_appear_in_same_query!(units, colors);
