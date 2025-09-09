#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialPositioningDeviceLotForeignKeys {
    pub commercial_positioning_device_lots_id_fkey: Option<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    >,
    pub commercial_positioning_device_lots_id_fkey1: Option<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >,
    pub product_model: Option<
        crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot {
    type ForeignKeys = CommercialPositioningDeviceLotForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                        self.id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDeviceModel(
                        self.id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceModel(
                        self.product_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.commercial_positioning_device_lots_id_fkey.is_some()
            && foreign_keys.commercial_positioning_device_lots_id_fkey1.is_some()
            && foreign_keys.product_model.is_some()
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
                crate::codegen::tables::row::Row::CommercialPositioningDeviceModel(
                    commercial_positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.product_model == commercial_positioning_device_models.id {
                    foreign_keys.product_model = Some(
                        commercial_positioning_device_models,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialPositioningDeviceModel(
                    commercial_positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.product_model == commercial_positioning_device_models.id {
                    foreign_keys.product_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProductLot(
                    commercial_product_lots,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == commercial_product_lots.id {
                    foreign_keys.commercial_positioning_device_lots_id_fkey = Some(
                        commercial_product_lots,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProductLot(
                    commercial_product_lots,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == commercial_product_lots.id {
                    foreign_keys.commercial_positioning_device_lots_id_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDeviceModel(
                    positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == positioning_device_models.id {
                    foreign_keys.commercial_positioning_device_lots_id_fkey1 = Some(
                        positioning_device_models,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PositioningDeviceModel(
                    positioning_device_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == positioning_device_models.id {
                    foreign_keys.commercial_positioning_device_lots_id_fkey1 = None;
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
impl web_common_traits::prelude::ForeignKeys for CommercialPositioningDeviceLotForeignKeys {}
