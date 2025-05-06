#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommercialProductLotForeignKeys {
    pub product_model:
        Option<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot
{
    type ForeignKeys = CommercialProductLotForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                self.product_model_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.product_model.is_some()
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
                crate::codegen::tables::row::Row::CommercialProduct(commercial_products),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if commercial_products.id == self.product_model_id {
                    foreign_keys.product_model = Some(commercial_products);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CommercialProduct(commercial_products),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if commercial_products.id == self.product_model_id {
                    foreign_keys.product_model = None;
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
impl web_common_traits::prelude::ForeignKeys for CommercialProductLotForeignKeys {}
