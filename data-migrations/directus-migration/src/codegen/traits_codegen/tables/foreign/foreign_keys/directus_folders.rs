#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectusFolderForeignKeys {
    pub parent: Option<
        std::rc::Rc<crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder>,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::directus_folders::DirectusFolder
{
    type ForeignKeys = DirectusFolderForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        if let Some(parent) = self.parent {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DirectusFolder(parent),
            ));
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.parent.is_some() || self.parent.is_none()
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
                if let Some(parent) = self.parent {
                    if directus_folders.id == parent {
                        foreign_keys.parent = Some(directus_folders);
                        updated = true;
                    }
                }
            }
            (
                crate::codegen::tables::row::Row::DirectusFolder(directus_folders),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if let Some(parent) = self.parent {
                    if directus_folders.id == parent {
                        foreign_keys.parent = None;
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
impl web_common_traits::prelude::ForeignKeys for DirectusFolderForeignKeys {}
