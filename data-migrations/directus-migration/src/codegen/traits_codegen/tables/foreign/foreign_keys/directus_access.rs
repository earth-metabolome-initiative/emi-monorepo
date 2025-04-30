#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusAccessForeignKeys {
    pub role:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>>,
    pub user:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub policy: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy>,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_access::DirectusAccess
{
    type ForeignKeys = DirectusAccessForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(role) = self.role {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusRole(role),
            ));
        }
        if let Some(user) = self.user {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(user),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusPolicy(
                self.policy,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.role.is_some() || self.role.is_none())
            && (foreign_keys.user.is_some() || self.user.is_none())
            && foreign_keys.policy.is_some()
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
                crate::codegen::tables::row::Row::DirectusPolicy(directus_policies),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if directus_policies.id == self.policy {
                    foreign_keys.policy = Some(directus_policies);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusPolicy(directus_policies),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if directus_policies.id == self.policy {
                    foreign_keys.policy = None;
                    updated = true;
                }
            }
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for DirectusAccessForeignKeys {}
