impl web_common_traits::prelude::Tabular for crate::Organism {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Organism
    }
}
impl web_common_traits::prelude::StaticTabular for crate::Organism {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::Organism
    }
}
impl web_common_traits::prelude::Row for crate::Organism {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(self.id)
    }
}
