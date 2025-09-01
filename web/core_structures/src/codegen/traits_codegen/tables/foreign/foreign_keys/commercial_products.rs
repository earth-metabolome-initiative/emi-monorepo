#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialProductForeignKeys {
    pub id:
        Option<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel>,
    pub brand: Option<crate::codegen::structs_codegen::tables::brands::Brand>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct
{
    type ForeignKeys = CommercialProductForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                self.id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(self.brand_id),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.brand.is_some()
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
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.brand_id == brands.id {
                    foreign_keys.brand = Some(brands);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Brand(brands),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.brand_id == brands.id {
                    foreign_keys.brand = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(physical_asset_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == physical_asset_models.id {
                    foreign_keys.id = Some(physical_asset_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAssetModel(physical_asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == physical_asset_models.id {
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
impl web_common_traits::prelude::ForeignKeys for CommercialProductForeignKeys {}
