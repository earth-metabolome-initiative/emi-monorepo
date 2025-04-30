#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContainerForeignKeys {
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub user_updated:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub container_model: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    >,
    pub location:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::universities::University>>,
    pub parent_container:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::containers::Container>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::containers::Container
{
    type ForeignKeys = ContainerForeignKeys;
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
        if let Some(container_model) = self.container_model {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                    container_model,
                ),
            ));
        }
        if let Some(location) = self.location {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::University(location),
            ));
        }
        if let Some(parent_container) = self.parent_container {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    parent_container,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user_created.is_some() || self.user_created.is_none())
            && (foreign_keys.user_updated.is_some() || self.user_updated.is_none())
            && (foreign_keys.container_model.is_some() || self.container_model.is_none())
            && (foreign_keys.location.is_some() || self.location.is_none())
            && (foreign_keys.parent_container.is_some() || self.parent_container.is_none())
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
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = Some(containers);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::Container(containers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(parent_container) = self.parent_container {
                    if containers.id == parent_container {
                        foreign_keys.parent_container = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(container_model) = self.container_model {
                    if container_models.id == container_model {
                        foreign_keys.container_model = Some(container_models);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(container_model) = self.container_model {
                    if container_models.id == container_model {
                        foreign_keys.container_model = None;
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
                crate::codegen::tables::row::Row::University(universities),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(location) = self.location {
                    if universities.id == location {
                        foreign_keys.location = Some(universities);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::University(universities),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(location) = self.location {
                    if universities.id == location {
                        foreign_keys.location = None;
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
impl web_common_traits::prelude::ForeignKeys for ContainerForeignKeys {}
