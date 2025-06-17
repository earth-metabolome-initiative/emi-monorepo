impl web_common_traits::prelude::Tabular
    for crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        crate::codegen::tables::table_names::TableName::BallMillContainerModel
    }
}
impl web_common_traits::prelude::StaticTabular
    for crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel
{
    fn static_table_name() -> Self::TableName {
        crate::codegen::tables::table_names::TableName::BallMillContainerModel
    }
}
impl web_common_traits::prelude::Row
    for crate::codegen::structs_codegen::tables::ball_mill_container_models::BallMillContainerModel
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillContainerModel((
            self.milled_with,
            self.container_model_id,
        ))
    }
}
