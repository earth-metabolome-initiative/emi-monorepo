impl web_common_traits::prelude::Tabular for crate::PackagingProcedure {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PackagingProcedure
    }
}
impl web_common_traits::prelude::StaticTabular for crate::PackagingProcedure {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PackagingProcedure
    }
}
impl web_common_traits::prelude::Row for crate::PackagingProcedure {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedure(
            self.procedure,
        )
    }
}
