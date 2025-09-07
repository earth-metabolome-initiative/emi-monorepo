impl web_common_traits::prelude::Tabular for crate::CommercialPositioningDeviceLot {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot
    }
}
impl web_common_traits::prelude::StaticTabular for crate::CommercialPositioningDeviceLot {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot
    }
}
impl web_common_traits::prelude::Row for crate::CommercialPositioningDeviceLot {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceLot(
            self.id,
        )
    }
}
