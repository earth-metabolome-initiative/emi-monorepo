#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusFileForeignKeys {
    pub folder: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
    >,
    pub uploaded_by:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
    pub modified_by:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_files::DirectusFile
{
    type ForeignKeys = DirectusFileForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(folder) = self.folder {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFolder(folder),
            ));
        }
        if let Some(uploaded_by) = self.uploaded_by {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    uploaded_by,
                ),
            ));
        }
        if let Some(modified_by) = self.modified_by {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusUser(
                    modified_by,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.folder.is_some() || self.folder.is_none())
            && (foreign_keys.uploaded_by.is_some() || self.uploaded_by.is_none())
            && (foreign_keys.modified_by.is_some() || self.modified_by.is_none())
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
                crate::codegen::tables::row::Row::DirectusFolder(directus_folders),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(folder) = self.folder {
                    if directus_folders.id == folder {
                        foreign_keys.folder = Some(directus_folders);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFolder(directus_folders),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(folder) = self.folder {
                    if directus_folders.id == folder {
                        foreign_keys.folder = None;
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
                if let Some(uploaded_by) = self.uploaded_by {
                    if directus_users.id == uploaded_by {
                        foreign_keys.uploaded_by = Some(directus_users.clone());
                        updated = true;
                    }
                }
                if let Some(modified_by) = self.modified_by {
                    if directus_users.id == modified_by {
                        foreign_keys.modified_by = Some(directus_users.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusUser(directus_users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(uploaded_by) = self.uploaded_by {
                    if directus_users.id == uploaded_by {
                        foreign_keys.uploaded_by = None;
                        updated = true;
                    }
                }
                if let Some(modified_by) = self.modified_by {
                    if directus_users.id == modified_by {
                        foreign_keys.modified_by = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusFileForeignKeys {}
