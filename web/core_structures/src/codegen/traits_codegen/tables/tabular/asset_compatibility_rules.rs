impl web_common_traits::prelude::Tabular for crate::AssetCompatibilityRule {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::AssetCompatibilityRule
    }
}
impl web_common_traits::prelude::StaticTabular for crate::AssetCompatibilityRule {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::AssetCompatibilityRule
    }
}
impl web_common_traits::prelude::Row for crate::AssetCompatibilityRule {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule((
            self.left_asset_model,
            self.right_asset_model,
        ))
    }
}
