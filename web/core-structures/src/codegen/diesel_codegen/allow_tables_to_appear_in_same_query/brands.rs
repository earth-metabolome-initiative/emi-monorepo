use crate::codegen::diesel_codegen::tables::{brand_states::brand_states, brands::brands};
diesel::allow_tables_to_appear_in_same_query!(brands, brand_states);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(brands, users);
