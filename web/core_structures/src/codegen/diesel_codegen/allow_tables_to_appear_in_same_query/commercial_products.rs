use crate::codegen::diesel_codegen::tables::brands::brands;
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, brands);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(commercial_products, trackables);
