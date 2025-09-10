#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContainerForeignKeys {
    pub container_model:
        Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    pub id: Option<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
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
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                self.container_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(self.id),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.container_model.is_some() && foreign_keys.id.is_some()
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
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.container_model == container_models.id {
                    foreign_keys.container_model = Some(container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.container_model == container_models.id {
                    foreign_keys.container_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == physical_assets.id {
                    foreign_keys.id = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == physical_assets.id {
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
impl web_common_traits::prelude::ForeignKeys for ContainerForeignKeys {}
