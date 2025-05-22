impl web_common_traits::prelude::Tabular
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepModelNameplateCategory
    }
}
impl web_common_traits::prelude::StaticTabular
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::StepModelNameplateCategory
    }
}
impl web_common_traits::prelude::Row
for crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::StepModelNameplateCategory(
            self.id,
        )
    }
}
