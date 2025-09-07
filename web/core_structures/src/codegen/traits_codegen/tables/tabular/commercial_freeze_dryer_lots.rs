impl web_common_traits::prelude::Tabular for crate::CommercialFreezeDryerLot {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot
    }
}
impl web_common_traits::prelude::StaticTabular for crate::CommercialFreezeDryerLot {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot
    }
}
impl web_common_traits::prelude::Row for crate::CommercialFreezeDryerLot {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezeDryerLot(
            self.id,
        )
    }
}
