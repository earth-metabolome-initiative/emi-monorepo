#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots
)]
pub struct CommercialProductLot {
    pub id: i32,
    pub lot: String,
    pub product_model_id: i32,
}
impl diesel::Identifiable for CommercialProductLot {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialProductLot {
    #[cfg(feature = "postgres")]
    pub async fn product_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::id
                    .eq(&self.product_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_product_model_id(
        conn: &mut diesel_async::AsyncPgConnection,
        product_model_id: &crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::dsl::product_model_id
                    .eq(product_model_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_lot_and_product_model_id(
        lot: &str,
        product_model_id: &i32,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
            associations::HasTable,
        };
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::lot
                    .eq(lot)
                    .and(
                        crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::product_model_id
                            .eq(product_model_id),
                    ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_lot(
        lot: &String,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots;
        Self::table()
            .filter(commercial_product_lots::lot.eq(lot))
            .order_by(commercial_product_lots::id.asc())
            .load::<Self>(conn)
            .await
    }
}
