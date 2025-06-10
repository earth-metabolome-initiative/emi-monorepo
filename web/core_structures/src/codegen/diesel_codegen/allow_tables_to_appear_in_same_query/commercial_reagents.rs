use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
use crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents;
diesel::allow_tables_to_appear_in_same_query!(commercial_reagents, commercial_product_lots);
use crate::codegen::diesel_codegen::tables::processables::processables;
diesel::allow_tables_to_appear_in_same_query!(commercial_reagents, processables);
use crate::codegen::diesel_codegen::tables::trackables::trackables;
diesel::allow_tables_to_appear_in_same_query!(commercial_reagents, trackables);
