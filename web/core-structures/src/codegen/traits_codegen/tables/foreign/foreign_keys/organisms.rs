#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrganismForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::trackables::Trackable>,
    pub nameplate_category:
        Option<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::organisms::Organism
{
    type ForeignKeys = OrganismForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Trackable(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::NameplateCategory(
                self.nameplate_category_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.nameplate_category.is_some()
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
                crate::codegen::tables::row::Row::NameplateCategory(nameplate_categories),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if nameplate_categories.id == self.nameplate_category_id {
                    foreign_keys.nameplate_category = Some(nameplate_categories);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::NameplateCategory(nameplate_categories),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if nameplate_categories.id == self.nameplate_category_id {
                    foreign_keys.nameplate_category = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if trackables.id == self.id {
                    foreign_keys.id = Some(trackables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Trackable(trackables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if trackables.id == self.id {
                    foreign_keys.id = None;
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
impl web_common_traits::prelude::ForeignKeys for OrganismForeignKeys {}
