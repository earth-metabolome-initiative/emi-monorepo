impl web_common_traits::prelude::Tabular for crate::PhotographProcedure {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PhotographProcedure
    }
}
impl web_common_traits::prelude::StaticTabular for crate::PhotographProcedure {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::PhotographProcedure
    }
}
impl web_common_traits::prelude::Row for crate::PhotographProcedure {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedure(
            self.procedure,
        )
    }
}
