#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusRevisionForeignKeys {
    pub activity: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_activity::DirectusActivity>,
    >,
    pub parent: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision>,
    >,
    pub version: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_versions::DirectusVersion>,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_revisions::DirectusRevision
{
    type ForeignKeys = DirectusRevisionForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusActivity(
                self.activity,
            ),
        ));
        if let Some(parent) = self.parent {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusRevision(
                    parent,
                ),
            ));
        }
        if let Some(version) = self.version {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusVersion(
                    version,
                ),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.activity.is_some()
            && (foreign_keys.parent.is_some() || self.parent.is_none())
            && (foreign_keys.version.is_some() || self.version.is_none())
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
                crate::codegen::tables::row::Row::DirectusRevision(directus_revisions),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(parent) = self.parent {
                    if directus_revisions.id == parent {
                        foreign_keys.parent = Some(directus_revisions);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusRevision(directus_revisions),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(parent) = self.parent {
                    if directus_revisions.id == parent {
                        foreign_keys.parent = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusVersion(directus_versions),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if let Some(version) = self.version {
                    if directus_versions.id == version {
                        foreign_keys.version = Some(directus_versions);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusVersion(directus_versions),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(version) = self.version {
                    if directus_versions.id == version {
                        foreign_keys.version = None;
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusActivity(directus_activity),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if directus_activity.id == self.activity {
                    foreign_keys.activity = Some(directus_activity);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusActivity(directus_activity),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if directus_activity.id == self.activity {
                    foreign_keys.activity = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusRevisionForeignKeys {}
