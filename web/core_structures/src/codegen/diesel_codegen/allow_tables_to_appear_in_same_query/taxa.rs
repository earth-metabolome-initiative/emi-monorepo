use crate::codegen::diesel_codegen::tables::{ranks::ranks, taxa::taxa};
diesel::allow_tables_to_appear_in_same_query!(taxa, ranks);
