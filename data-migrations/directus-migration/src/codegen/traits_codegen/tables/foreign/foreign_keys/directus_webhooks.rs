#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusWebhookForeignKeys {
    pub migrated_flow:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_webhooks::DirectusWebhook
{
    type ForeignKeys = DirectusWebhookForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(migrated_flow) = self.migrated_flow {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFlow(
                    migrated_flow,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.migrated_flow.is_some() || self.migrated_flow.is_none()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::DirectusFlow(directus_flows),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(migrated_flow) = self.migrated_flow {
                    if directus_flows.id == migrated_flow {
                        foreign_keys.migrated_flow = Some(directus_flows);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFlow(directus_flows),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(migrated_flow) = self.migrated_flow {
                    if directus_flows.id == migrated_flow {
                        foreign_keys.migrated_flow = None;
                        updated = true;
                    }
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for DirectusWebhookForeignKeys {}
