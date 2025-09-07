#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialBallMillMachineLotForeignKeys {
    pub commercial_ball_mill_machine_lots_id_fkey: Option<crate::CommercialProductLot>,
    pub commercial_ball_mill_machine_lots_id_fkey1: Option<crate::BallMillMachineModel>,
    pub product_model: Option<crate::CommercialBallMillMachineModel>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::CommercialBallMillMachineLot {
    type ForeignKeys = CommercialBallMillMachineLotForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                self.id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                self.id,
            ),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBallMillMachineModel(
                        self.product_model,
                    ),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.commercial_ball_mill_machine_lots_id_fkey.is_some()
            && foreign_keys.commercial_ball_mill_machine_lots_id_fkey1.is_some()
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
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == ball_mill_machine_models.id {
                    foreign_keys.commercial_ball_mill_machine_lots_id_fkey1 =
                        Some(ball_mill_machine_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::BallMillMachineModel(ball_mill_machine_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == ball_mill_machine_models.id {
                    foreign_keys.commercial_ball_mill_machine_lots_id_fkey1 = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialBallMillMachineModel(
                    commercial_ball_mill_machine_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.product_model == commercial_ball_mill_machine_models.id {
                    foreign_keys.product_model = Some(commercial_ball_mill_machine_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialBallMillMachineModel(
                    commercial_ball_mill_machine_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.product_model == commercial_ball_mill_machine_models.id {
                    foreign_keys.product_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProductLot(commercial_product_lots),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.id == commercial_product_lots.id {
                    foreign_keys.commercial_ball_mill_machine_lots_id_fkey =
                        Some(commercial_product_lots);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProductLot(commercial_product_lots),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.id == commercial_product_lots.id {
                    foreign_keys.commercial_ball_mill_machine_lots_id_fkey = None;
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
impl web_common_traits::prelude::ForeignKeys for CommercialBallMillMachineLotForeignKeys {}
