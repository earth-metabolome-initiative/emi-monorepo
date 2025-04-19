use crate::codegen::diesel_codegen::tables::{
    brands::brands, commercial_products::commercial_products,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_products, brands);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, users);
use crate::codegen::diesel_codegen::tables::photographs::photographs;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, photographs);
