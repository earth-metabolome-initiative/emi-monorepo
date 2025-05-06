#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrackableLocationForeignKeys {
    pub trackable: Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
    pub storage_container:
        Option<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation
{
    type ForeignKeys = TrackableLocationForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(
                self.trackable_id,
            ),
        ));
        if let Some(storage_container_id) = self.storage_container_id {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageContainer(
                    storage_container_id,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.trackable.is_some()
            && (foreign_keys.storage_container.is_some() || self.storage_container_id.is_none())
            && foreign_keys.created_by.is_some()
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
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if trackables.id == self.trackable_id {
                    foreign_keys.trackable = Some(trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if trackables.id == self.trackable_id {
                    foreign_keys.trackable = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::StorageContainer(storage_containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(storage_container_id) = self.storage_container_id {
                    if storage_containers.id == storage_container_id {
                        foreign_keys.storage_container = Some(storage_containers);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::StorageContainer(storage_containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(storage_container_id) = self.storage_container_id {
                    if storage_containers.id == storage_container_id {
                        foreign_keys.storage_container = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = Some(users);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if users.id == self.created_by {
                    foreign_keys.created_by = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for TrackableLocationForeignKeys {}
