use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
use crate::codegen::diesel_codegen::tables::commercial_products::commercial_products;
diesel::allow_tables_to_appear_in_same_query!(commercial_product_lots, commercial_products);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(commercial_product_lots, trackables);
