impl web_common_traits::prelude::Tabular for crate::TeamMember {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::TeamMember
    }
}
impl web_common_traits::prelude::StaticTabular for crate::TeamMember {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::TeamMember
    }
}
impl web_common_traits::prelude::Row for crate::TeamMember {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember((
            self.team_id,
            self.member_id,
        ))
    }
}
