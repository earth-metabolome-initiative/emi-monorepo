#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusCollectionForeignKeys {
    pub group: Option<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        >,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection
{
    type ForeignKeys = DirectusCollectionForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(group) = self.group {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusCollection(
                    group,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.group.is_some() || self.group.is_none()
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
                crate::codegen::tables::row::Row::DirectusCollection(directus_collections),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(group) = self.group {
                    if directus_collections.collection == group {
                        foreign_keys.group = Some(directus_collections);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusCollection(directus_collections),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(group) = self.group {
                    if directus_collections.collection == group {
                        foreign_keys.group = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusCollectionForeignKeys {}
