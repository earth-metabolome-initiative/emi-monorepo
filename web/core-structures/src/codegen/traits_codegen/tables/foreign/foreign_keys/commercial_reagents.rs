#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialReagentForeignKeys {
    pub id: Option<crate::codegen::structs_codegen::tables::processables::Processable>,
    pub commercial_product_lot: Option<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent
{
    type ForeignKeys = CommercialReagentForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Processable(self.id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                self.commercial_product_lot_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.id.is_some() && foreign_keys.commercial_product_lot.is_some()
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
                crate::codegen::tables::row::Row::CommercialProductLot(commercial_product_lots),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if commercial_product_lots.id == self.commercial_product_lot_id {
                    foreign_keys.commercial_product_lot = Some(commercial_product_lots);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProductLot(commercial_product_lots),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if commercial_product_lots.id == self.commercial_product_lot_id {
                    foreign_keys.commercial_product_lot = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if processables.id == self.id {
                    foreign_keys.id = Some(processables);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Processable(processables),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if processables.id == self.id {
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
impl web_common_traits::prelude::ForeignKeys for CommercialReagentForeignKeys {}
