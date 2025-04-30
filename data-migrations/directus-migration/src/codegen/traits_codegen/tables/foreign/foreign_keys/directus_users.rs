#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusUserForeignKeys {
    pub role:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_users::DirectusUser
{
    type ForeignKeys = DirectusUserForeignKeys;
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
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.role.is_some() || self.role.is_none()
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
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for DirectusUserForeignKeys {}
