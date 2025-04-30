#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusSettingForeignKeys {
    pub project_logo:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>>,
    pub public_foreground:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>>,
    pub public_background:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>>,
    pub storage_default_folder: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
    >,
    pub public_favicon:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_files::DirectusFile>>,
    pub public_registration_role:
        Option<std::rc::Rc<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_settings::DirectusSetting
{
    type ForeignKeys = DirectusSettingForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(project_logo) = self.project_logo {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFile(
                    project_logo,
                ),
            ));
        }
        if let Some(public_foreground) = self.public_foreground {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFile(
                    public_foreground,
                ),
            ));
        }
        if let Some(public_background) = self.public_background {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFile(
                    public_background,
                ),
            ));
        }
        if let Some(storage_default_folder) = self.storage_default_folder {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFolder(
                    storage_default_folder,
                ),
            ));
        }
        if let Some(public_favicon) = self.public_favicon {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFile(
                    public_favicon,
                ),
            ));
        }
        if let Some(public_registration_role) = self.public_registration_role {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusRole(
                    public_registration_role,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        (foreign_keys.project_logo.is_some() || self.project_logo.is_none())
            && (foreign_keys.public_foreground.is_some() || self.public_foreground.is_none())
            && (foreign_keys.public_background.is_some() || self.public_background.is_none())
            && (foreign_keys.storage_default_folder.is_some()
                || self.storage_default_folder.is_none())
            && (foreign_keys.public_favicon.is_some() || self.public_favicon.is_none())
            && (foreign_keys.public_registration_role.is_some()
                || self.public_registration_role.is_none())
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
                crate::codegen::tables::row::Row::DirectusFile(directus_files),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(project_logo) = self.project_logo {
                    if directus_files.id == project_logo {
                        foreign_keys.project_logo = Some(directus_files.clone());
                        updated = true;
                    }
                }
                if let Some(public_foreground) = self.public_foreground {
                    if directus_files.id == public_foreground {
                        foreign_keys.public_foreground = Some(directus_files.clone());
                        updated = true;
                    }
                }
                if let Some(public_background) = self.public_background {
                    if directus_files.id == public_background {
                        foreign_keys.public_background = Some(directus_files.clone());
                        updated = true;
                    }
                }
                if let Some(public_favicon) = self.public_favicon {
                    if directus_files.id == public_favicon {
                        foreign_keys.public_favicon = Some(directus_files.clone());
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFile(directus_files),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(project_logo) = self.project_logo {
                    if directus_files.id == project_logo {
                        foreign_keys.project_logo = None;
                        updated = true;
                    }
                }
                if let Some(public_foreground) = self.public_foreground {
                    if directus_files.id == public_foreground {
                        foreign_keys.public_foreground = None;
                        updated = true;
                    }
                }
                if let Some(public_background) = self.public_background {
                    if directus_files.id == public_background {
                        foreign_keys.public_background = None;
                        updated = true;
                    }
                }
                if let Some(public_favicon) = self.public_favicon {
                    if directus_files.id == public_favicon {
                        foreign_keys.public_favicon = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFolder(directus_folders),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(storage_default_folder) = self.storage_default_folder {
                    if directus_folders.id == storage_default_folder {
                        foreign_keys.storage_default_folder = Some(directus_folders);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFolder(directus_folders),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(storage_default_folder) = self.storage_default_folder {
                    if directus_folders.id == storage_default_folder {
                        foreign_keys.storage_default_folder = None;
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
                if let Some(public_registration_role) = self.public_registration_role {
                    if directus_roles.id == public_registration_role {
                        foreign_keys.public_registration_role = Some(directus_roles);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusRole(directus_roles),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(public_registration_role) = self.public_registration_role {
                    if directus_roles.id == public_registration_role {
                        foreign_keys.public_registration_role = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusSettingForeignKeys {}
