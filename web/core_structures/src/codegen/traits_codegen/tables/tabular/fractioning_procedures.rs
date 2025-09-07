impl web_common_traits::prelude::Tabular for crate::FractioningProcedure {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FractioningProcedure
    }
}
impl web_common_traits::prelude::StaticTabular for crate::FractioningProcedure {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::FractioningProcedure
    }
}
impl web_common_traits::prelude::Row for crate::FractioningProcedure {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedure(
            self.procedure,
        )
    }
}
