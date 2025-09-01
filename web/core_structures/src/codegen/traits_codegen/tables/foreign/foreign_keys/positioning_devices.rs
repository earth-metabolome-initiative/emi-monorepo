#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositioningDeviceForeignKeys {
    pub id: Option<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    >,
    pub model: Option<
        crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice
{
    type ForeignKeys = PositioningDeviceForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(self.id),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceLot(
                        self.model_id,
                    ),
                ),
            );
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
                crate::codegen::tables::row::Row::CommercialPositioningDeviceLot(
                    commercial_positioning_device_lots,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.model_id == commercial_positioning_device_lots.id {
                    foreign_keys.model = Some(commercial_positioning_device_lots);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialPositioningDeviceLot(
                    commercial_positioning_device_lots,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.model_id == commercial_positioning_device_lots.id {
                    foreign_keys.model = None;
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
impl web_common_traits::prelude::ForeignKeys for PositioningDeviceForeignKeys {}
