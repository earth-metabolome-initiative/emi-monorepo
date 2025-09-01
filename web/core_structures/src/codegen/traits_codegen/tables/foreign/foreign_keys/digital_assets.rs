#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DigitalAssetForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::assets::Asset>,
    pub model:
        Option<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset
{
    type ForeignKeys = DigitalAssetForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Asset(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAssetModel(
                self.model_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.model.is_some()
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
                crate::codegen::tables::row::Row::Asset(assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == assets.id {
                    foreign_keys.id = Some(assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Asset(assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == assets.id {
                    foreign_keys.id = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DigitalAssetModel(digital_asset_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.model_id == digital_asset_models.id {
                    foreign_keys.model = Some(digital_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::DigitalAssetModel(digital_asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.model_id == digital_asset_models.id {
                    foreign_keys.model = None;
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
impl web_common_traits::prelude::ForeignKeys for DigitalAssetForeignKeys {}
