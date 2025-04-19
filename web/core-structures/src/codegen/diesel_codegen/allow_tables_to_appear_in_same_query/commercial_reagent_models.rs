use crate::codegen::diesel_codegen::tables::{
    commercial_products::commercial_products, commercial_reagent_models::commercial_reagent_models,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_reagent_models, commercial_products);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(commercial_reagent_models, users);
