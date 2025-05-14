use crate::codegen::diesel_codegen::tables::{
    commercial_reagents::commercial_reagents, processables::processables,
};
diesel::allow_tables_to_appear_in_same_query!(commercial_reagents, processables);
use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
diesel::allow_tables_to_appear_in_same_query!(commercial_reagents, commercial_product_lots);
