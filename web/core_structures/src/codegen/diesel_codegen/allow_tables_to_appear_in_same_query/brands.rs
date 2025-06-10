use crate::codegen::diesel_codegen::tables::brands::brands;
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(brands, users);
