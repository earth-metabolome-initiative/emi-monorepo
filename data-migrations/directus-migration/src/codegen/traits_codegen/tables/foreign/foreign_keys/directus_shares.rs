#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusShareForeignKeys {
    pub collection: Option<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        >,
    >,
    pub role:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>>,
    pub user_created:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_shares::DirectusShare
{
    type ForeignKeys = DirectusShareForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusCollection(
                self.collection,
            ),
        ));
        if let Some(role) = self.role {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusRole(role),
            ));
        }
        if let Some(user_created) = self.user_created {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    user_created,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.collection.is_some()
            && (foreign_keys.role.is_some() || self.role.is_none())
            && (foreign_keys.user_created.is_some() || self.user_created.is_none())
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
                crate::codegen::tables::row::Row::DirectusRole(directus_roles),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(role) = self.role {
                    if directus_roles.id == role {
                        foreign_keys.role = Some(directus_roles);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusRole(directus_roles),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(role) = self.role {
                    if directus_roles.id == role {
                        foreign_keys.role = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusCollection(directus_collections),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if directus_collections.collection == self.collection {
                    foreign_keys.collection = Some(directus_collections);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusCollection(directus_collections),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if directus_collections.collection == self.collection {
                    foreign_keys.collection = None;
                    updated = true;
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
                        foreign_keys.user_created = Some(directus_users);
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
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for DirectusShareForeignKeys {}
