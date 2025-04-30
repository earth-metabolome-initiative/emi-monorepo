#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DriedSamplesDatumForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub sample_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
    pub parent_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
    pub batch: Option<std::rc::Rc<crate::codegen::structs_codegen::tables::batches::Batch>>,
    pub field_data:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::field_data::FieldDatum>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::dried_samples_data::DriedSamplesDatum
{
    type ForeignKeys = DriedSamplesDatumForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(user_created) = self.user_created {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_created,
                ),
            ));
        }
        if let Some(user_updated) = self.user_updated {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_updated,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                self.sample_container,
            ),
        ));
        if let Some(parent_container) = self.parent_container {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    parent_container,
                ),
            ));
        }
        if let Some(batch) = self.batch {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Batch(batch),
            ));
        }
        if let Some(field_data) = self.field_data {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FieldDatum(field_data),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && foreign_keys.sample_container.is_some()
            && (foreign_keys.parent_container.is_some() || self.parent_container.is_none())
            && (foreign_keys.batch.is_some() || self.batch.is_none())
            && (foreign_keys.field_data.is_some() || self.field_data.is_none())
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
                crate::codegen::tables::row::Row::FieldDatum(field_data),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(field_data) = self.field_data {
                    if field_data.id == field_data {
                        foreign_keys.field_data = Some(field_data);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::FieldDatum(field_data),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(field_data) = self.field_data {
                    if field_data.id == field_data {
                        foreign_keys.field_data = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = Some(directus_users.clone());
                        updated = true;
                    }
                }
                if let Some(user_updated) = self.user_updated {
                    if directus_users.id == user_updated {
                        foreign_keys.user_updated = Some(directus_users.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(user_created) = self.user_created {
                    if directus_users.id == user_created {
                        foreign_keys.user_created = None;
                        updated = true;
                    }
                }
                if let Some(user_updated) = self.user_updated {
                    if directus_users.id == user_updated {
                        foreign_keys.user_updated = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if containers.id == self.sample_container {
                    foreign_keys.sample_container = Some(containers.clone());
                    updated = true;
                }
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = Some(containers.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if containers.id == self.sample_container {
                    foreign_keys.sample_container = None;
                    updated = true;
                }
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Batch(batches),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(batch) = self.batch {
                    if batches.id == batch {
                        foreign_keys.batch = Some(batches);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Batch(batches),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(batch) = self.batch {
                    if batches.id == batch {
                        foreign_keys.batch = None;
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
impl web_common_traits::prelude::ForeignKeys for DriedSamplesDatumForeignKeys {}
