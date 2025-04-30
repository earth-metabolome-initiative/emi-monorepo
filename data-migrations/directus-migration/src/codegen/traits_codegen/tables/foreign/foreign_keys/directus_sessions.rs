#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusSessionForeignKeys {
    pub user:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub share: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_shares::DirectusShare>,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_sessions::DirectusSession
{
    type ForeignKeys = DirectusSessionForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(user) = self.user {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(user),
            ));
        }
        if let Some(share) = self.share {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusShare(share),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.user.is_some() || self.user.is_none())
            && (foreign_keys.share.is_some() || self.share.is_none())
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
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(user) = self.user {
                    if directus_users.id == user {
                        foreign_keys.user = Some(directus_users);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(user) = self.user {
                    if directus_users.id == user {
                        foreign_keys.user = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusShare(directus_shares),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(share) = self.share {
                    if directus_shares.id == share {
                        foreign_keys.share = Some(directus_shares);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusShare(directus_shares),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(share) = self.share {
                    if directus_shares.id == share {
                        foreign_keys.share = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusSessionForeignKeys {}
