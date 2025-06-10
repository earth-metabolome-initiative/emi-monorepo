use crate::codegen::diesel_codegen::tables::ranks::ranks;
use crate::codegen::diesel_codegen::tables::taxa::taxa;
diesel::allow_tables_to_appear_in_same_query!(taxa, ranks);
